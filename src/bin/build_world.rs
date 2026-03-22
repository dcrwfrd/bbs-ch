/// build_world — ingests docs/schools.csv and generates data/schools.json
/// and data/conferences.json. Both files are always regenerated from the CSV.
///
/// Run from the project root:
///   cargo run --bin build_world
///
/// Expected CSV columns (see docs/schools.csv):
///   id, name, nickname, abbreviation, prefer_abbreviation,
///   zip_code, primary_color, secondary_color,
///   conference, home_court, arena_capacity, cumulative_success

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use bbs_ch::model::{
    conference::{Conference, Region},
    prestige::{ConferenceTier, ProgramPrestige},
    school::{HomeCourt, School},
};

fn main() -> Result<()> {
    let csv_path = "docs/schools.csv";
    let confs_path = "data/conferences.json";
    let schools_path = "data/schools.json";

    // ------------------------------------------------------------------
    // Parse schools.
    // ------------------------------------------------------------------
    let mut rdr = csv::Reader::from_path(csv_path)
        .with_context(|| format!("Cannot open {csv_path}"))?;
    let mut schools: Vec<School> = Vec::new();

    for result in rdr.records() {
        let r = result?;

        let raw_name               = r.get(1).unwrap_or("").trim();
        let nickname               = r.get(2).unwrap_or("").trim().to_string();
        let abbreviation           = r.get(3).unwrap_or("").trim().to_string();
        let prefer_abbreviation    = r.get(4).unwrap_or("false").trim() == "true";
        let zip_code               = r.get(5).unwrap_or("").trim().to_string();
        let primary_color          = normalize_hex(r.get(6).unwrap_or(""));
        let secondary_color        = normalize_hex(r.get(7).unwrap_or(""));
        let conf_name              = r.get(8).unwrap_or("").trim().to_string();
        let home_court_name        = r.get(9).unwrap_or("").trim().to_string();
        let arena_capacity: u32    = r.get(10).unwrap_or("0").trim().parse().unwrap_or(0);
        let cumulative_success: u8 = r.get(11).unwrap_or("50").trim().parse().unwrap_or(50);

        let name = colloquial_name(raw_name);
        // Store the conference abbreviation as the human-readable cross-reference.
        let conference = infer_abbreviation(&conf_name);
        let atmosphere = capacity_to_atmosphere(arena_capacity);

        schools.push(School {
            id: 0, // assigned by DataLoader at runtime
            name,
            nickname,
            abbreviation,
            prefer_abbreviation,
            zip_code,
            conference,
            conference_id: 0, // resolved by DataLoader at runtime
            primary_color,
            secondary_color,
            home_court: HomeCourt {
                name: home_court_name,
                capacity: arena_capacity,
                atmosphere,
            },
            prestige: ProgramPrestige {
                cumulative_success,
                recent_success: 50,
                facilities: 50,
                resources: 50,
                atmosphere,
                coaching_staff: 50,
                market: 50,
                media_perception: 50,
                academics: 50,
            },
            enrollment: 0,
            nil_budget: default_nil_budget(cumulative_success),
            primary_rival: None,
            rivals: Vec::new(),
        });
    }

    // ------------------------------------------------------------------
    // Build conferences from the school list.
    // Collect unique conference full names, build Conference structs with
    // school abbreviations as the member list.
    // ------------------------------------------------------------------
    let mut conf_name_to_members: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for school in &schools {
        // school.conference already holds the abbreviation; we need the full
        // name to build the Conference struct. Recover it from the original
        // abbreviation via a reverse lookup.
        let conf_name = abbr_to_conf_name(&school.conference);
        conf_name_to_members
            .entry(conf_name)
            .or_default()
            .push(school.abbreviation.clone());
    }

    let mut conferences: Vec<Conference> = conf_name_to_members
        .into_iter()
        .map(|(name, mut members)| {
            members.sort();
            Conference {
                id: 0, // assigned by DataLoader at runtime
                abbreviation: infer_abbreviation(&name),
                region: infer_region(&name),
                tier: infer_tier(&name),
                member_ids: vec![], // resolved by DataLoader at runtime
                members,
                name,
                logo: None,
            }
        })
        .collect();
    conferences.sort_by(|a, b| a.name.cmp(&b.name));

    // ------------------------------------------------------------------
    // Write output.
    // ------------------------------------------------------------------
    std::fs::create_dir_all("data").context("Cannot create data/")?;

    let schools_json = serde_json::to_string_pretty(&schools)
        .context("Failed to serialize schools")?;
    std::fs::write(schools_path, &schools_json)
        .context("Failed to write data/schools.json")?;
    println!("Wrote {} schools to {schools_path}.", schools.len());

    let confs_json = serde_json::to_string_pretty(&conferences)
        .context("Failed to serialize conferences")?;
    std::fs::write(confs_path, &confs_json)
        .context("Failed to write data/conferences.json")?;
    println!("Wrote {} conferences to {confs_path}.", conferences.len());

    Ok(())
}

// ---------------------------------------------------------------------------
// Name transformation
// ---------------------------------------------------------------------------

/// Convert a full institutional name to the colloquial team name.
fn colloquial_name(raw: &str) -> String {
    // Explicit overrides for names that don't follow a simple pattern.
    let overrides: &[(&str, &str)] = &[
        ("University at Albany",          "Albany"),
        ("University at Albany SUNY",     "Albany"),
        ("Binghamton University",          "Binghamton"),
        ("Bryant University",              "Bryant"),
        ("University of Maine",            "Maine"),
        ("University of Maryland, Baltimore County", "UMBC"),
        ("University of Massachusetts Lowell", "UMass Lowell"),
        ("University of New Hampshire",    "New Hampshire"),
        ("New Jersey Institute of Technology", "NJIT"),
        ("University of Vermont",          "Vermont"),
        ("University of North Carolina at Charlotte", "Charlotte"),
        ("East Carolina University",       "East Carolina"),
        ("Florida Atlantic University",    "Florida Atlantic"),
        ("University of Memphis",          "Memphis"),
        ("University of North Texas",      "North Texas"),
        ("Rice University",                "Rice"),
        ("University of South Florida",    "South Florida"),
        ("Temple University",              "Temple"),
        ("University of Alabama at Birmingham", "UAB"),
        ("Tulane University",              "Tulane"),
        ("Wichita State University",       "Wichita State"),
        ("University of Cincinnati",       "Cincinnati"),
        ("University of Florida",          "Florida"),
        ("Florida State University",       "Florida State"),
        ("Georgia Institute of Technology","Georgia Tech"),
        ("University of Louisville",       "Louisville"),
        ("University of Miami",            "Miami"),
        ("North Carolina State University","NC State"),
        ("University of Notre Dame",       "Notre Dame"),
        ("University of Pittsburgh",       "Pittsburgh"),
        ("Syracuse University",            "Syracuse"),
        ("Virginia Polytechnic Institute", "Virginia Tech"),
        ("Wake Forest University",         "Wake Forest"),
        ("University of Virginia",         "Virginia"),
        ("Duke University",                "Duke"),
        ("University of North Carolina",   "North Carolina"),
        ("Clemson University",             "Clemson"),
        ("University of Georgia",          "Georgia"),
        ("Louisiana State University",     "Louisiana State"),
        ("University of Alabama",          "Alabama"),
        ("Auburn University",              "Auburn"),
        ("University of South Carolina",   "South Carolina"),
        ("University of Kentucky",         "Kentucky"),
        ("University of Tennessee",        "Tennessee"),
        ("Vanderbilt University",          "Vanderbilt"),
        ("Mississippi State University",   "Mississippi State"),
        ("University of Mississippi",      "Ole Miss"),
        ("University of Arkansas",         "Arkansas"),
        ("University of Missouri",         "Missouri"),
        ("University of Texas at Austin",  "Texas"),
        ("University of Texas",            "Texas"),
        ("Texas A&M University",           "Texas A&M"),
        ("Oklahoma State University",      "Oklahoma State"),
        ("University of Oklahoma",         "Oklahoma"),
        ("Baylor University",              "Baylor"),
        ("Texas Christian University",     "TCU"),
        ("Texas Tech University",          "Texas Tech"),
        ("University of Kansas",           "Kansas"),
        ("Kansas State University",        "Kansas State"),
        ("Iowa State University",          "Iowa State"),
        ("University of Iowa",             "Iowa"),
        ("University of Michigan",         "Michigan"),
        ("Michigan State University",      "Michigan State"),
        ("Ohio State University",          "Ohio State"),
        ("Penn State University",          "Penn State"),
        ("Rutgers University",             "Rutgers"),
        ("University of Maryland",         "Maryland"),
        ("University of Minnesota",        "Minnesota"),
        ("University of Wisconsin",        "Wisconsin"),
        ("University of Illinois",         "Illinois"),
        ("Northwestern University",        "Northwestern"),
        ("Purdue University",              "Purdue"),
        ("Indiana University",             "Indiana"),
        ("University of Nebraska",         "Nebraska"),
        ("University of Oregon",           "Oregon"),
        ("Oregon State University",        "Oregon State"),
        ("University of Washington",       "Washington"),
        ("Washington State University",    "Washington State"),
        ("University of Arizona",          "Arizona"),
        ("Arizona State University",       "Arizona State"),
        ("University of California",       "California"),
        ("University of California, Los Angeles", "UCLA"),
        ("University of Southern California", "USC"),
        ("Stanford University",            "Stanford"),
        ("University of Utah",             "Utah"),
        ("University of Colorado",         "Colorado"),
        ("Colorado State University",      "Colorado State"),
        ("Utah State University",          "Utah State"),
        ("Brigham Young University",       "BYU"),
        ("University of Nevada",           "Nevada"),
        ("University of Nevada, Las Vegas","UNLV"),
        ("Fresno State University",        "Fresno State"),
        ("San Diego State University",     "San Diego State"),
        ("Boise State University",         "Boise State"),
        ("University of New Mexico",       "New Mexico"),
        ("New Mexico State University",    "New Mexico State"),
        ("University of Wyoming",          "Wyoming"),
        ("Air Force Academy",              "Air Force"),
        ("United States Naval Academy",    "Navy"),
        ("United States Military Academy", "Army"),
        ("Georgetown University",          "Georgetown"),
        ("Villanova University",           "Villanova"),
        ("St. John's University",          "St. John's"),
        ("Marquette University",           "Marquette"),
        ("DePaul University",              "DePaul"),
        ("Seton Hall University",          "Seton Hall"),
        ("Providence College",             "Providence"),
        ("Butler University",              "Butler"),
        ("Xavier University",              "Xavier"),
        ("Creighton University",           "Creighton"),
        ("University of Connecticut",      "UConn"),
        ("Duke University",                "Duke"),
        ("Gonzaga University",             "Gonzaga"),
        ("Saint Mary's College",           "Saint Mary's"),
        ("Loyola Marymount University",    "Loyola Marymount"),
        ("University of San Francisco",    "San Francisco"),
        ("Santa Clara University",         "Santa Clara"),
        ("Pepperdine University",          "Pepperdine"),
        ("University of Portland",         "Portland"),
        ("Pacific University",             "Pacific"),
        ("Virginia Commonwealth University","VCU"),
        ("College of William & Mary",      "William & Mary"),
        ("University of Richmond",         "Richmond"),
        ("George Mason University",        "George Mason"),
        ("James Madison University",       "James Madison"),
        ("Old Dominion University",        "Old Dominion"),
        ("Hofstra University",             "Hofstra"),
        ("Northeastern University",        "Northeastern"),
        ("University of Delaware",         "Delaware"),
        ("Drexel University",              "Drexel"),
        ("College of Charleston",          "Charleston"),
        ("Elon University",                "Elon"),
        ("Stony Brook University",         "Stony Brook"),
        ("Towson University",              "Towson"),
        ("University of North Carolina at Wilmington", "UNCW"),
        ("Southern Methodist University",  "SMU"),
        ("University of Texas at El Paso", "UTEP"),
        ("University of Texas at San Antonio", "UTSA"),
        ("University of Texas at Arlington","UT Arlington"),
        ("Florida International University","FIU"),
        ("Florida Atlantic University",    "Florida Atlantic"),
        ("University of Alabama in Huntsville", "UAH"),
        ("Western Kentucky University",    "Western Kentucky"),
        ("Middle Tennessee State University","Middle Tennessee"),
        ("East Tennessee State University","East Tennessee State"),
        ("Morehead State University",      "Morehead State"),
        ("Murray State University",        "Murray State"),
        ("Southeast Missouri State University","Southeast Missouri"),
        ("Tennessee State University",     "Tennessee State"),
        ("Tennessee Tech University",      "Tennessee Tech"),
        ("University of Tennessee at Martin","Tennessee Martin"),
        ("Bellarmine University",          "Bellarmine"),
        ("Jacksonville State University",  "Jacksonville State"),
        ("Liberty University",             "Liberty"),
        ("Longwood University",            "Longwood"),
        ("North Alabama University",       "North Alabama"),
        ("Queens University of Charlotte", "Queens"),
        ("Southern Indiana University",    "Southern Indiana"),
        ("Tarleton State University",      "Tarleton State"),
        ("Utah Tech University",           "Utah Tech"),
        ("Utah Valley University",         "Utah Valley"),
        ("Stephen F. Austin State University", "SFA"),
        ("Lamar University",               "Lamar"),
        ("University of the Incarnate Word","Incarnate Word"),
        ("Northwestern State University",  "Northwestern State"),
        ("Nicholls State University",      "Nicholls"),
        ("McNeese State University",       "McNeese"),
        ("Southeastern Louisiana University","Southeastern Louisiana"),
        ("University of Louisiana at Lafayette", "Louisiana"),
        ("University of Louisiana at Monroe",    "Louisiana Monroe"),
        ("University of New Orleans",      "New Orleans"),
        ("Texas Southern University",      "Texas Southern"),
        ("Prairie View A&M University",    "Prairie View"),
        ("Grambling State University",     "Grambling"),
        ("Southern University",            "Southern"),
        ("Alcorn State University",        "Alcorn State"),
        ("Jackson State University",       "Jackson State"),
        ("Mississippi Valley State University","Mississippi Valley"),
        ("Alabama A&M University",         "Alabama A&M"),
        ("Alabama State University",       "Alabama State"),
        ("Bethune-Cookman University",     "Bethune-Cookman"),
        ("Coppin State University",        "Coppin State"),
        ("Delaware State University",      "Delaware State"),
        ("Florida A&M University",         "Florida A&M"),
        ("Howard University",              "Howard"),
        ("Maryland Eastern Shore University","Maryland-Eastern Shore"),
        ("Morgan State University",        "Morgan State"),
        ("Norfolk State University",       "Norfolk State"),
        ("North Carolina A&T State University","NC A&T"),
        ("North Carolina Central University","NC Central"),
        ("South Carolina State University","SC State"),
        ("Savannah State University",      "Savannah State"),
        ("University of Arkansas at Pine Bluff","Arkansas-Pine Bluff"),
        ("Long Island University",         "LIU"),
        ("Fairleigh Dickinson University", "Fairleigh Dickinson"),
        ("Saint Francis University",       "St. Francis (PA)"),
        ("Wagner College",                 "Wagner"),
        ("Mount St. Mary's University",    "Mount St. Mary's"),
        ("Central Connecticut State University","Central Connecticut"),
        ("Sacred Heart University",        "Sacred Heart"),
        ("Robert Morris University",       "Robert Morris"),
        ("Saint Peter's University",       "Saint Peter's"),
        ("Rider University",               "Rider"),
        ("Niagara University",             "Niagara"),
        ("Canisius University",            "Canisius"),
        ("Monmouth University",            "Monmouth"),
        ("Quinnipiac University",          "Quinnipiac"),
        ("Manhattan College",              "Manhattan"),
        ("Iona University",                "Iona"),
        ("Fairfield University",           "Fairfield"),
        ("Siena College",                  "Siena"),
        ("Marist College",                 "Marist"),
        ("University of Kentucky",         "Kentucky"),
        ("Western Michigan University",    "Western Michigan"),
        ("Eastern Michigan University",    "Eastern Michigan"),
        ("Central Michigan University",    "Central Michigan"),
        ("Northern Illinois University",   "Northern Illinois"),
        ("Ball State University",          "Ball State"),
        ("Bowling Green State University", "Bowling Green"),
        ("Ohio University",                "Ohio"),
        ("Miami University",               "Miami (OH)"),
        ("Kent State University",          "Kent State"),
        ("Akron University",               "Akron"),
        ("Toledo University",              "Toledo"),
        ("Buffalo University",             "Buffalo"),
        ("University of Massachusetts",    "UMass"),
        ("University of Rhode Island",     "Rhode Island"),
        ("Saint Louis University",         "Saint Louis"),
        ("Dayton University",              "Dayton"),
        ("University of Dayton",           "Dayton"),
        ("Duquesne University",            "Duquesne"),
        ("Saint Joseph's University",      "Saint Joseph's"),
        ("La Salle University",            "La Salle"),
        ("George Washington University",   "George Washington"),
        ("Fordham University",             "Fordham"),
        ("Davidson College",               "Davidson"),
        ("University of San Diego",        "San Diego"),
        ("Loyola University Chicago",      "Loyola Chicago"),
        ("University of Illinois at Chicago","Illinois-Chicago"),
        ("Northern Iowa University",       "Northern Iowa"),
        ("Bradley University",             "Bradley"),
        ("Southern Illinois University",   "SIU"),
        ("Missouri State University",      "Missouri State"),
        ("Indiana State University",       "Indiana State"),
        ("Evansville University",          "Evansville"),
        ("Drake University",               "Drake"),
        ("Valparaiso University",          "Valparaiso"),
        ("Illinois State University",      "Illinois State"),
        ("Harvard University",             "Harvard"),
        ("Yale University",                "Yale"),
        ("Princeton University",           "Princeton"),
        ("Cornell University",             "Cornell"),
        ("Dartmouth College",              "Dartmouth"),
        ("Brown University",               "Brown"),
        ("Columbia University",            "Columbia"),
        ("University of Pennsylvania",     "Penn"),
        ("College of the Holy Cross",      "Holy Cross"),
        ("Bucknell University",            "Bucknell"),
        ("Colgate University",             "Colgate"),
        ("Lehigh University",              "Lehigh"),
        ("Lafayette College",              "Lafayette"),
        ("American University",            "American"),
        ("Boston University",              "Boston University"),
        ("Loyola University Maryland",     "Loyola Maryland"),
        ("University of Denver",           "Denver"),
        ("North Dakota State University",  "North Dakota State"),
        ("South Dakota State University",  "South Dakota State"),
        ("Oral Roberts University",        "Oral Roberts"),
        ("University of South Dakota",     "South Dakota"),
        ("University of North Dakota",     "North Dakota"),
        ("Western Illinois University",    "Western Illinois"),
        ("Fort Wayne University",          "Fort Wayne"),
        ("University of Wisconsin-Green Bay","Green Bay"),
        ("University of Wisconsin-Milwaukee","Milwaukee"),
        ("Oakland University",             "Oakland"),
        ("Wright State University",        "Wright State"),
        ("Youngstown State University",    "Youngstown State"),
        ("Cleveland State University",     "Cleveland State"),
        ("IUPUI",                          "IUPUI"),
        ("Northern Kentucky University",   "Northern Kentucky"),
        ("Belmont University",             "Belmont"),
        ("Samford University",             "Samford"),
        ("The Citadel",                    "The Citadel"),
        ("Wofford College",                "Wofford"),
        ("Furman University",              "Furman"),
        ("Mercer University",              "Mercer"),
        ("Western Carolina University",    "Western Carolina"),
        ("VMI",                            "VMI"),
        ("University of North Carolina at Greensboro","UNC Greensboro"),
        ("Chattanooga University",         "Chattanooga"),
        ("East Tennessee State University","ETSU"),
        ("Appalachian State University",   "Appalachian State"),
        ("University of Texas at Arlington","UT Arlington"),
    ];

    for (full, short) in overrides {
        if raw == *full {
            return short.to_string();
        }
    }

    // Fallback: strip common prefixes like "University of " and " University".
    let s = raw
        .trim_start_matches("University of ")
        .trim_start_matches("The University of ")
        .trim_end_matches(" University")
        .trim_end_matches(" College")
        .trim_end_matches(" State University")
        .trim();
    s.to_string()
}

// ---------------------------------------------------------------------------
// Helpers (unchanged from original)
// ---------------------------------------------------------------------------

/// Normalize a color input to a lowercase hex string (e.g. `"#003087"`).
/// Returns `""` if the input is blank or invalid — indicates not yet filled in.
fn normalize_hex(s: &str) -> String {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 { return String::new(); }
    // Validate that all chars are hex digits.
    if !s.chars().all(|c| c.is_ascii_hexdigit()) { return String::new(); }
    format!("#{}", s.to_lowercase())
}

/// Reverse map: conference abbreviation → full name.
/// Needed when recovering the full name from the abbreviation we stored on School.
fn abbr_to_conf_name(abbr: &str) -> String {
    match abbr {
        "ACC"   => "ACC",            "AE"    => "America East",
        "AAC"   => "American",       "A10"   => "Atlantic 10",
        "ASUN"  => "Atlantic Sun",   "B12"   => "Big 12",
        "BE"    => "Big East",       "BSky"  => "Big Sky",
        "BSC"   => "Big South",      "B1G"   => "Big Ten",
        "BWC"   => "Big West",       "CAA"   => "CAA",
        "CUSA"  => "Conference USA", "HL"    => "Horizon League",
        "IVY"   => "Ivy League",     "MAAC"  => "MAAC",
        "MAC"   => "MAC",            "MEAC"  => "MEAC",
        "MVC"   => "Missouri Valley","MWC"   => "Mountain West",
        "NEC"   => "Northeast",      "OVC"   => "Ohio Valley",
        "PL"    => "Patriot League", "SEC"   => "SEC",
        "SoCon" => "Southern",       "SLC"   => "Southland",
        "SWAC"  => "SWAC",           "SUM"   => "Summit League",
        "SBC"   => "Sun Belt",       "WAC"   => "WAC",
        "WCC"   => "West Coast",
        other   => other,
    }.to_string()
}

fn capacity_to_atmosphere(capacity: u32) -> u8 {
    match capacity {
        0..=2000      => 35,
        2001..=4000   => 42,
        4001..=7000   => 50,
        7001..=10000  => 58,
        10001..=15000 => 65,
        15001..=20000 => 72,
        _             => 78,
    }
}

fn default_nil_budget(cumulative_success: u8) -> u32 {
    match cumulative_success {
        90..=99 => 8_000_000,
        75..=89 => 5_000_000,
        55..=74 => 2_500_000,
        35..=54 => 1_000_000,
        _       =>   300_000,
    }
}

fn infer_abbreviation(name: &str) -> String {
    match name {
        "ACC"             => "ACC",    "America East"    => "AE",
        "American"        => "AAC",    "Atlantic 10"     => "A10",
        "Atlantic Sun"    => "ASUN",   "Big 12"          => "B12",
        "Big East"        => "BE",     "Big Sky"         => "BSky",
        "Big South"       => "BSC",    "Big Ten"         => "B1G",
        "Big West"        => "BWC",    "CAA"             => "CAA",
        "Conference USA"  => "CUSA",   "Horizon League"  => "HL",
        "Ivy League"      => "IVY",    "MAAC"            => "MAAC",
        "MAC"             => "MAC",    "MEAC"            => "MEAC",
        "Missouri Valley" => "MVC",    "Mountain West"   => "MWC",
        "Northeast"       => "NEC",    "Ohio Valley"     => "OVC",
        "Patriot League"  => "PL",     "SEC"             => "SEC",
        "Southern"        => "SoCon",  "Southland"       => "SLC",
        "SWAC"            => "SWAC",   "Summit League"   => "SUM",
        "Sun Belt"        => "SBC",    "WAC"             => "WAC",
        "West Coast"      => "WCC",
        _ => &name[..name.len().min(6)],
    }.to_string()
}

fn infer_region(name: &str) -> Region {
    match name {
        "ACC" | "Southern" | "Big South" | "MEAC" | "Atlantic Sun"
        | "Sun Belt" | "SEC"                             => Region::Southeast,
        "America East" | "Ivy League" | "Patriot League"
        | "MAAC" | "Northeast"                           => Region::Northeast,
        "Big East" | "Atlantic 10" | "MAC" | "CAA"      => Region::MidAtlantic,
        "Big Ten" | "Horizon League" | "Missouri Valley"
        | "Summit League" | "Ohio Valley"                => Region::Midwest,
        "American" | "Conference USA" | "Southland"
        | "SWAC"                                         => Region::Southwest,
        "Big West" | "Mountain West" | "WAC" | "West Coast"
        | "Big Sky"                                      => Region::West,
        "Big 12"                                         => Region::Plains,
        _                                                => Region::Midwest,
    }
}

fn infer_tier(name: &str) -> ConferenceTier {
    match name {
        "ACC" | "Big Ten" | "Big 12" | "SEC" | "Big East" => ConferenceTier::PowerConference,
        "Mountain West" | "Atlantic 10" | "American" | "West Coast"
        | "Missouri Valley" | "MAC" | "CAA" | "MAAC" | "Atlantic Sun"
        | "Horizon League" | "Sun Belt" | "Conference USA" | "Southern" => ConferenceTier::MidMajor,
        _ => ConferenceTier::LowMajor,
    }
}
