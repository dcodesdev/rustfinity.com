use return_trait_objects::*;

fn should_impl_speakable<T: Speakable>() {}

#[test]
fn test_dog_speaker() {
    let speaker = get_speaker("dog");
    assert_eq!(speaker.speak(), "Woof");
}

#[test]
fn test_robot_speaker() {
    let speaker = get_speaker("robot");
    assert_eq!(speaker.speak(), "Beep boop");
}

#[test]
#[should_panic]
fn test_invalid_speaker_type() {
    get_speaker("alien");
}

#[test]
fn test_dog_impl_speakable() {
    should_impl_speakable::<Dog>();
}

#[test]
fn test_robot_impl_speakable() {
    should_impl_speakable::<Robot>();
}

#[test]
fn test_correct_implementation() {
    let dog = Dog {
        name: "Rex".to_string(),
        breed: "Labrador".to_string(),
    };

    assert_eq!(dog.speak(), "Woof");

    let robot = Robot {
        model: "R2-D2".to_string(),
        purpose: "assistance".to_string(),
    };

    assert_eq!(robot.speak(), "Beep boop");
}
