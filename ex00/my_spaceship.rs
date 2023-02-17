struct Spacechip {
    x: i32,
    y: i32,
    direction: &'static str,
}

fn my_spaceship(space_way: &str) -> String {
    let mut spacechip = Spacechip {
        x: 0,
        y: 0,
        direction: "up",
    };

    for ch in space_way.chars() {
        if ch == 'R' && spacechip.direction == "up" {
            spacechip.direction = "right";
        } else if ch == 'R' && spacechip.direction == "right" {
            spacechip.direction = "down";
        } else if ch == 'R' && spacechip.direction == "down" {
            spacechip.direction = "left";
        } else if ch == 'R' && spacechip.direction == "left" {
            spacechip.direction = "up";
        } else if ch == 'L' && spacechip.direction == "up" {
            spacechip.direction = "left";
        } else if ch == 'L' && spacechip.direction == "left" {
            spacechip.direction = "down";
        } else if ch == 'L' && spacechip.direction == "down" {
            spacechip.direction = "right";
        } else if ch == 'L' && spacechip.direction == "right" {
            spacechip.direction = "up";
        } else if ch == 'A' && spacechip.direction == "up" {
            spacechip.y -= 1;
        } else if ch == 'A' && spacechip.direction == "left" {
            spacechip.x -= 1;
        } else if ch == 'A' && spacechip.direction == "down" {
            spacechip.y += 1;
        } else if ch == 'A' && spacechip.direction == "right" {
            spacechip.x += 1;
        }
    }

    format!("{{x: {}, y: {}, direction: '{}'}}", spacechip.x, spacechip.y, spacechip.direction)
}

