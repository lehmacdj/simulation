use frame;
use rules;

#[test]
fn frame_init() {
    let frame = frame::Frame::<i32>::new(10, 10);
    assert_eq!(frame.width(), 10);
    assert_eq!(frame.height(), 10);
    for x in 0..9 {
        for y in 0..9 {
            assert_eq!(*frame.get(x, y), i32::default());
        }
    }
}

#[test]
fn frame_mut() {
    let mut frame = frame::Frame::<i32>::new(2, 2);
    frame.set(1, 1, 1);
    assert_eq!(*frame.get(1, 1), 1)
}

#[test]
fn frame_next() {
    let mut frame1 = frame::Frame::<i32>::new(2, 2);

    let frame2 = frame1.next_frame(|sq| { sq.get(0, 0) + 1 });

    let val = i32::default() + 1;
    frame1.set(0, 0, val);
    frame1.set(0, 1, val);
    frame1.set(1, 0, val);
    frame1.set(1, 1, val);

    assert_eq!(frame1, frame2);
}

#[test]
fn game_of_life() {
    use rules::GOLState;
    use rules::GOLState::Alive;

    // Create a board that should transform as shown:
    // DAD    DDD    DAD
    // DAD -> AAA -> DAD
    // DAD    DDD    DAD
    // the board is padded a lot because otherwise the overflow at the edges
    // would mess with the simulation
    let mut frame1 = frame::Frame::<GOLState>::new(4, 4);
    frame1.set(1, 0, Alive);
    frame1.set(1, 1, Alive);
    frame1.set(1, 2, Alive);

    let frame2 = frame1.next_frame(rules::game_of_life);
    let frame3 = frame2.next_frame(rules::game_of_life);

    let mut expected = frame::Frame::<GOLState>::new(4, 4);
    expected.set(0, 1, Alive);
    expected.set(1, 1, Alive);
    expected.set(2, 1, Alive);

    assert_eq!(frame2, expected);
    assert_eq!(frame3, frame1);
}
