fn compile_to_html(pts: &str) {
    let mut result = String::new();

    for line in pts.lines() {
        if line.starts_with("#") {
            let mut idx = 0;
            while line.chars().nth(idx).unwrap() == '#' {
                idx += 1;
            }
            let remaining_text = line.split("").collect()[idx..line.len()];
        }
    }
}
