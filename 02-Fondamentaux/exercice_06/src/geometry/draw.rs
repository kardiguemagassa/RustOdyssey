pub fn draw_triangle(height: u32) {
    for i in 0..height {
        let spaces = height - i - 1;
        let stars = 2 * i + 1;

        println!(
            "{}{}",
            " ".repeat(spaces as usize),
            "*".repeat(stars as usize)
        );
    }
}
