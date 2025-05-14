#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}

#[derive(Clone)]
struct Event {
    x: i32,
    y1: i32,
    y2: i32,
    typ: i32,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    let mut events = vec![];

    for r in rects {
        let x1 = r.a.x.min(r.b.x);
        let x2 = r.a.x.max(r.b.x);
        let y1 = r.a.y.min(r.b.y);
        let y2 = r.a.y.max(r.b.y);

        events.push(Event { x: x1, y1, y2, typ: 1 });
        events.push(Event { x: x2, y1, y2, typ: -1 });
    }

    events.sort_by_key(|e| e.x);

    let mut y_intervals: Vec<(i32, i32)> = vec![];
    let mut prev_x = events[0].x;
    let mut total_area = 0;

    for e in events {
        let mut covered_y = 0;
        let mut last_y = -1;
        let mut active = vec![];

        for (start, end) in &y_intervals {
            active.push((*start, *end));
        }
        active.sort();

        for (start, end) in active {
            let s = start.max(last_y);
            if end > s {
                covered_y += end - s;
                last_y = end;
            }
        }

        let width = e.x - prev_x;
        total_area += width * covered_y;

        if e.typ == 1 {
            y_intervals.push((e.y1, e.y2));
        } else {
            y_intervals.retain(|&(y1, y2)| y1 != e.y1 || y2 != e.y2);
        }

        prev_x = e.x;
    }

    total_area
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Test passed: occupied area = {}", occupied);
}

fn main() {
    area_occupied_test();
}
