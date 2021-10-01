fn coint<T>(
    y0: Vec<T>,
    y1: Vec<T>,
    trend: Option<&str>,
    method: Option<&str>,
    maxlag: Option<&str>,
    autolag: Option<&str>,
    return_results: Option<&str>,
) -> Result<(), String> {
    if y0.len() != y1.len() {
        return Err("Mismatched length of vectors".to_string());
    }
    Err(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bad_length() {
        match coint(vec![0; 5], vec![0; 4], None, None, None, None, None) {
            Ok(_) => panic!("Did not throw panic"),
            Err(e) => {
                if e != "Mismatched length of vectors".to_string() {
                    panic!("Wrong error: '{}'", e);
                }
            }
        }
    }
}
