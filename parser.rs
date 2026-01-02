use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref NAME_PATTERN: Regex = Regex::new(r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+){1,2})\b").unwrap();
    static ref STATE_PATTERN: Regex = Regex::new(
        r"\b(AL|AK|AZ|AR|CA|CO|CT|DE|FL|GA|HI|ID|IL|IN|IA|KS|KY|LA|ME|MD|MA|MI|MN|MS|MO|MT|NE|NV|NH|NJ|NM|NY|NC|ND|OH|OK|OR|PA|RI|SC|SD|TN|TX|UT|VT|VA|WA|WV|WI|WY)\b"
    ).unwrap();
    static ref CITY_STATE_PATTERN: Regex = Regex::new(
        r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)?),?\s+(AL|AK|AZ|AR|CA|CO|CT|DE|FL|GA|HI|ID|IL|IN|IA|KS|KY|LA|ME|MD|MA|MI|MN|MS|MO|MT|NE|NV|NH|NJ|NM|NY|NC|ND|OH|OK|OR|PA|RI|SC|SD|TN|TX|UT|VT|VA|WA|WV|WI|WY)\b"
    ).unwrap();
    static ref ZIP_PATTERN: Regex = Regex::new(r"\b\d{5}(?:-\d{4})?\b").unwrap();
    
    static ref EXCLUDED_WORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("Phone");
        set.insert("Number");
        set.insert("Call");
        set.insert("Contact");
        set.insert("Email");
        set.insert("Address");
        set.insert("Street");
        set.insert("City");
        set.insert("State");
        set.insert("Country");
        set.insert("The");
        set.insert("This");
        set.insert("That");
        set.insert("Search");
        set.insert("Results");
        set.insert("View");
        set.insert("More");
        set.insert("Less");
        set.insert("Show");
        set.insert("Hide");
        set.insert("United States");
        set.insert("New York");
        set.insert("Los Angeles");
        set.insert("San Francisco");
        set.insert("Google");
        set.insert("Bing");
        set.insert("Yahoo");
        set.insert("Facebook");
        set.insert("Twitter");
        set.insert("Instagram");
        set.insert("Best");
        set.insert("Top");
        set.insert("Free");
        set.insert("Online");
        set.insert("Reviews");
        set.insert("About");
        set.insert("Home");
        set.insert("Business");
        set.insert("Service");
        set.insert("Services");
        set.insert("Company");
        set.insert("Companies");
        set
    };

    static ref US_STATES: Vec<(&'static str, &'static str)> = vec![
        ("AL", "Alabama"),
        ("AK", "Alaska"),
        ("AZ", "Arizona"),
        ("AR", "Arkansas"),
        ("CA", "California"),
        ("CO", "Colorado"),
        ("CT", "Connecticut"),
        ("DE", "Delaware"),
        ("FL", "Florida"),
        ("GA", "Georgia"),
        ("HI", "Hawaii"),
        ("ID", "Idaho"),
        ("IL", "Illinois"),
        ("IN", "Indiana"),
        ("IA", "Iowa"),
        ("KS", "Kansas"),
        ("KY", "Kentucky"),
        ("LA", "Louisiana"),
        ("ME", "Maine"),
        ("MD", "Maryland"),
        ("MA", "Massachusetts"),
        ("MI", "Michigan"),
        ("MN", "Minnesota"),
        ("MS", "Mississippi"),
        ("MO", "Missouri"),
        ("MT", "Montana"),
        ("NE", "Nebraska"),
        ("NV", "Nevada"),
        ("NH", "New Hampshire"),
        ("NJ", "New Jersey"),
        ("NM", "New Mexico"),
        ("NY", "New York"),
        ("NC", "North Carolina"),
        ("ND", "North Dakota"),
        ("OH", "Ohio"),
        ("OK", "Oklahoma"),
        ("OR", "Oregon"),
        ("PA", "Pennsylvania"),
        ("RI", "Rhode Island"),
        ("SC", "South Carolina"),
        ("SD", "South Dakota"),
        ("TN", "Tennessee"),
        ("TX", "Texas"),
        ("UT", "Utah"),
        ("VT", "Vermont"),
        ("VA", "Virginia"),
        ("WA", "Washington"),
        ("WV", "West Virginia"),
        ("WI", "Wisconsin"),
        ("WY", "Wyoming"),
    ];
}

pub fn extract_names(text: &str) -> Vec<String> {
    let mut names = Vec::new();

    for cap in NAME_PATTERN.captures_iter(text) {
        if let Some(name_match) = cap.get(1) {
            let name = name_match.as_str();
            
            // Filter out excluded words and single-word names
            if !EXCLUDED_WORDS.contains(name) && name.split_whitespace().count() >= 2 {
                names.push(name.to_string());
            }
        }
    }

    names
}

pub fn extract_locations(text: &str) -> Vec<String> {
    let mut locations = Vec::new();

    // Extract state abbreviations
    for cap in STATE_PATTERN.captures_iter(text) {
        if let Some(state) = cap.get(1) {
            locations.push(state.as_str().to_string());
        }
    }

    // Extract city, state combinations
    for cap in CITY_STATE_PATTERN.captures_iter(text) {
        if let (Some(city), Some(state)) = (cap.get(1), cap.get(2)) {
            locations.push(format!("{}, {}", city.as_str(), state.as_str()));
        }
    }

    // Extract full state names
    for (_, full_name) in US_STATES.iter() {
        if text.contains(full_name) {
            locations.push(full_name.to_string());
        }
    }

    // Extract zip codes
    for cap in ZIP_PATTERN.captures_iter(text) {
        if let Some(zip) = cap.get(0) {
            locations.push(zip.as_str().to_string());
        }
    }

    // Deduplicate
    let unique_locations: HashSet<String> = locations.into_iter().collect();
    unique_locations.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_names() {
        let text = "Contact John Smith or Jane Doe for more information.";
        let names = extract_names(text);
        assert!(names.contains(&"John Smith".to_string()));
        assert!(names.contains(&"Jane Doe".to_string()));
    }

    #[test]
    fn test_extract_locations() {
        let text = "Located in Philadelphia, PA 19102";
        let locations = extract_locations(text);
        assert!(locations.contains(&"PA".to_string()));
        assert!(locations.contains(&"Philadelphia, PA".to_string()));
        assert!(locations.contains(&"19102".to_string()));
    }

    #[test]
    fn test_filter_excluded_words() {
        let text = "Phone Number Contact Email";
        let names = extract_names(text);
        assert!(names.is_empty());
    }
}
