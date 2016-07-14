use super::Interval;

/// Tests the Interval constructors for points.
#[test]
fn interval_point_constructors() {
    let o: fn(f32, f32) -> Interval<f32> = Interval::open;
    let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
    let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
    let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

    // Point constructors.
    assert!(o(0.5, 0.5).is_empty());
    assert_eq!(lo(0.5, 0.5), c(0.5, 0.5));
    assert_eq!(ro(0.5, 0.5), c(0.5, 0.5));
    assert_eq!(c(0.5, 0.5), c(0.5, 0.5));
}

/// Tests the Interval::intersect function.
#[test]
fn interval_contains() {
    let int = Interval::right_open(0.0, 2.0);
    assert!(!int.contains(&-1.34));
    assert!(!int.contains(&-0.001));
    assert!(int.contains(&0.0));
    assert!(int.contains(&0.001));
    assert!(int.contains(&1.0));
    assert!(int.contains(&1.9999));
    assert!(!int.contains(&2.0));
}

/// Tests the Interval::intersect function.
#[test]
fn interval_intersect() {
    let o: fn(f32, f32) -> Interval<f32> = Interval::open;
    let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
    let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
    let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

    // Open overlapping.
    assert_eq!( o(1.0, 2.0).intersect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).intersect(&lo(1.0, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).intersect(&ro(1.0, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).intersect(& c(1.0, 2.0)), Some( o(1.0, 2.0)));

    // Closed overlapping.
    assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 2.0)), Some(lo(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 2.0)), Some(ro(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 2.0)), Some( c(1.0, 2.0)));
    
    // Open left-half overlapping.
    assert_eq!( o(1.0, 2.0).intersect(& o(1.0, 1.5)), Some( o(1.0, 1.5)));
    assert_eq!( o(1.0, 2.0).intersect(&lo(1.0, 1.5)), Some(lo(1.0, 1.5)));
    assert_eq!( o(1.0, 2.0).intersect(&ro(1.0, 1.5)), Some( o(1.0, 1.5)));
    assert_eq!( o(1.0, 2.0).intersect(& c(1.0, 1.5)), Some(lo(1.0, 1.5)));

    // Close left-half overlapping.
    assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 1.5)), Some( o(1.0, 1.5)));
    assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 1.5)), Some(lo(1.0, 1.5)));
    assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 1.5)), Some(ro(1.0, 1.5)));
    assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 1.5)), Some( c(1.0, 1.5)));

    // Open right-half overlapping.
    assert_eq!( o(1.0, 2.0).intersect(& o(1.5, 2.0)), Some( o(1.5, 2.0)));
    assert_eq!( o(1.0, 2.0).intersect(&lo(1.5, 2.0)), Some( o(1.5, 2.0)));
    assert_eq!( o(1.0, 2.0).intersect(&ro(1.5, 2.0)), Some(ro(1.5, 2.0)));
    assert_eq!( o(1.0, 2.0).intersect(& c(1.5, 2.0)), Some(ro(1.5, 2.0)));

    // Closed right-half overlapping.
    assert_eq!( c(1.0, 2.0).intersect(& o(1.5, 2.0)), Some( o(1.5, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(&lo(1.5, 2.0)), Some(lo(1.5, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(&ro(1.5, 2.0)), Some(ro(1.5, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(& c(1.5, 2.0)), Some( c(1.5, 2.0)));

    // Open Subset overlapping.
    assert_eq!( o(1.0, 2.0).intersect(& o(1.2, 1.8)), Some( o(1.2, 1.8)));
    assert_eq!( o(1.0, 2.0).intersect(&lo(1.2, 1.8)), Some(lo(1.2, 1.8)));
    assert_eq!( o(1.0, 2.0).intersect(&ro(1.2, 1.8)), Some(ro(1.2, 1.8)));
    assert_eq!( o(1.0, 2.0).intersect(& c(1.2, 1.8)), Some( c(1.2, 1.8)));

    // Closed Subset overlapping.
    assert_eq!( c(1.0, 2.0).intersect(& o(1.2, 1.8)), Some( o(1.2, 1.8)));
    assert_eq!( c(1.0, 2.0).intersect(&lo(1.2, 1.8)), Some(lo(1.2, 1.8)));
    assert_eq!( c(1.0, 2.0).intersect(&ro(1.2, 1.8)), Some(ro(1.2, 1.8)));
    assert_eq!( c(1.0, 2.0).intersect(& c(1.2, 1.8)), Some( c(1.2, 1.8)));

    // Right non-overlapping.
    assert_eq!( o(1.0, 2.0).intersect(& o(2.0, 3.0)), None);
    assert_eq!( o(1.0, 2.0).intersect(&lo(2.0, 3.0)), None);
    assert_eq!( o(1.0, 2.0).intersect(&ro(2.0, 3.0)), None);
    assert_eq!( o(1.0, 2.0).intersect(& c(2.0, 3.0)), None);

    // Left non-overlapping.
    assert_eq!( o(1.0, 2.0).intersect(& o(0.0, 1.0)), None);
    assert_eq!( o(1.0, 2.0).intersect(&lo(0.0, 1.0)), None);
    assert_eq!( o(1.0, 2.0).intersect(&ro(0.0, 1.0)), None);
    assert_eq!( o(1.0, 2.0).intersect(& c(0.0, 1.0)), None);

    // Center Point intersections.
    assert_eq!( o(1.0, 2.0).intersect(& o(1.5, 1.5)), None);
    assert_eq!( o(1.0, 2.0).intersect(&lo(1.5, 1.5)), Some( c(1.5, 1.5)));
    assert_eq!( o(1.0, 2.0).intersect(&ro(1.5, 1.5)), Some( c(1.5, 1.5)));
    assert_eq!( o(1.0, 2.0).intersect(& c(1.5, 1.5)), Some( c(1.5, 1.5)));

    // Left Point intersections.
    assert_eq!( c(1.0, 2.0).intersect(& o(1.0, 1.0)), None);
    assert_eq!( c(1.0, 2.0).intersect(&lo(1.0, 1.0)), Some( c(1.0, 1.0)));
    assert_eq!( c(1.0, 2.0).intersect(&ro(1.0, 1.0)), Some( c(1.0, 1.0)));
    assert_eq!( c(1.0, 2.0).intersect(& c(1.0, 1.0)), Some( c(1.0, 1.0)));

    // Right Point intersections.
    assert_eq!( c(1.0, 2.0).intersect(& o(2.0, 2.0)), None);
    assert_eq!( c(1.0, 2.0).intersect(&lo(2.0, 2.0)), Some( c(2.0, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(&ro(2.0, 2.0)), Some( c(2.0, 2.0)));
    assert_eq!( c(1.0, 2.0).intersect(& c(2.0, 2.0)), Some( c(2.0, 2.0)));
}


/// Tests the Interval::union function.
#[test]
fn interval_union() {
    let o: fn(f32, f32) -> Interval<f32> = Interval::open;
    let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
    let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
    let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

    // Open overlapping.
    assert_eq!( o(1.0, 2.0).union(& o(1.0, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(1.0, 2.0)), Some(lo(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(1.0, 2.0)), Some(ro(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(1.0, 2.0)), Some( c(1.0, 2.0)));

    // Closed overlapping.
    assert_eq!( c(1.0, 2.0).union(& o(1.0, 2.0)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&lo(1.0, 2.0)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&ro(1.0, 2.0)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(& c(1.0, 2.0)), Some( c(1.0, 2.0)));
    
    // Open left-half overlapping.
    assert_eq!( o(1.0, 2.0).union(& o(1.0, 1.5)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(1.0, 1.5)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(1.0, 1.5)), Some(ro(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(1.0, 1.5)), Some(ro(1.0, 2.0)));

    // Close left-half overlapping.
    assert_eq!( c(1.0, 2.0).union(& o(1.0, 1.5)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&lo(1.0, 1.5)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&ro(1.0, 1.5)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(& c(1.0, 1.5)), Some( c(1.0, 2.0)));

    // Open right-half overlapping.
    assert_eq!( o(1.0, 2.0).union(& o(1.5, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(1.5, 2.0)), Some(lo(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(1.5, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(1.5, 2.0)), Some(lo(1.0, 2.0)));

    // Closed right-half overlapping.
    assert_eq!( c(1.0, 2.0).union(& o(1.5, 2.0)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&lo(1.5, 2.0)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&ro(1.5, 2.0)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(& c(1.5, 2.0)), Some( c(1.0, 2.0)));

    // Open Subset overlapping.
    assert_eq!( o(1.0, 2.0).union(& o(1.2, 1.8)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(1.2, 1.8)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(1.2, 1.8)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(1.2, 1.8)), Some( o(1.0, 2.0)));

    // Closed Subset overlapping.
    assert_eq!( c(1.0, 2.0).union(& o(1.2, 1.8)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&lo(1.2, 1.8)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(&ro(1.2, 1.8)), Some( c(1.0, 2.0)));
    assert_eq!( c(1.0, 2.0).union(& c(1.2, 1.8)), Some( c(1.0, 2.0)));

    // Right non-overlapping.
    assert_eq!( o(1.0, 2.0).union(& o(2.0, 3.0)), None);
    assert_eq!( o(1.0, 2.0).union(&lo(2.0, 3.0)), None);
    assert_eq!( o(1.0, 2.0).union(&ro(2.0, 3.0)), Some( o(1.0, 3.0)));
    assert_eq!( o(1.0, 2.0).union(& c(2.0, 3.0)), Some(lo(1.0, 3.0)));

    // Left non-overlapping.
    assert_eq!( o(1.0, 2.0).union(& o(0.0, 1.0)), None);
    assert_eq!( o(1.0, 2.0).union(&lo(0.0, 1.0)), Some( o(0.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(0.0, 1.0)), None);
    assert_eq!( o(1.0, 2.0).union(& c(0.0, 1.0)), Some(ro(0.0, 2.0)));

    // Center Point unions.
    assert_eq!( o(1.0, 2.0).union(& o(1.5, 1.5)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(1.5, 1.5)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(1.5, 1.5)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(1.5, 1.5)), Some( o(1.0, 2.0)));

    // Left Point unions.
    assert_eq!( o(1.0, 2.0).union(& o(1.0, 1.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(1.0, 1.0)), Some(ro(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(1.0, 1.0)), Some(ro(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(1.0, 1.0)), Some(ro(1.0, 2.0)));

    // Right Point unions.
    assert_eq!( o(1.0, 2.0).union(& o(2.0, 2.0)), Some( o(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&lo(2.0, 2.0)), Some(lo(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(&ro(2.0, 2.0)), Some(lo(1.0, 2.0)));
    assert_eq!( o(1.0, 2.0).union(& c(2.0, 2.0)), Some(lo(1.0, 2.0)));
}

/// Tests the Interval::connect function.
#[test]
fn interval_connect() {
    use super::Interval as I;
    let o: fn(f32, f32) -> Interval<f32> = Interval::open;
    let c: fn(f32, f32) -> Interval<f32> = Interval::closed;
    let lo: fn(f32, f32) -> Interval<f32> = Interval::left_open;
    let ro: fn(f32, f32) -> Interval<f32> = Interval::right_open;

    // Open overlapping.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(1.0, 2.0)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(1.0, 2.0)]), Some(lo(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(1.0, 2.0)]), Some(ro(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(1.0, 2.0)]), Some( c(1.0, 2.0)));

    // Closed overlapping.
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  o(1.0, 2.0)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), lo(1.0, 2.0)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), ro(1.0, 2.0)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  c(1.0, 2.0)]), Some( c(1.0, 2.0)));
    
    // Open left-half overlapping.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(1.0, 1.5)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(1.0, 1.5)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(1.0, 1.5)]), Some(ro(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(1.0, 1.5)]), Some(ro(1.0, 2.0)));

    // Close left-half overlapping.
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  o(1.0, 1.5)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), lo(1.0, 1.5)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), ro(1.0, 1.5)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  c(1.0, 1.5)]), Some( c(1.0, 2.0)));

    // Open right-half overlapping.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(1.5, 2.0)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(1.5, 2.0)]), Some(lo(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(1.5, 2.0)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(1.5, 2.0)]), Some(lo(1.0, 2.0)));

    // Closed right-half overlapping.
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  o(1.5, 2.0)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), lo(1.5, 2.0)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), ro(1.5, 2.0)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  c(1.5, 2.0)]), Some( c(1.0, 2.0)));

    // Open Subset overlapping.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(1.2, 1.8)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(1.2, 1.8)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(1.2, 1.8)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(1.2, 1.8)]), Some( o(1.0, 2.0)));

    // Closed Subset overlapping.
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  o(1.2, 1.8)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), lo(1.2, 1.8)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0), ro(1.2, 1.8)]), Some( c(1.0, 2.0)));
    assert_eq!(I::enclose(vec![c(1.0, 2.0),  c(1.2, 1.8)]), Some( c(1.0, 2.0)));

    // Right non-overlapping.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(2.0, 3.0)]), Some( o(1.0, 3.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(2.0, 3.0)]), Some(lo(1.0, 3.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(2.0, 3.0)]), Some( o(1.0, 3.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(2.0, 3.0)]), Some(lo(1.0, 3.0)));

    // Left non-overlapping.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(0.0, 1.0)]), Some( o(0.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(0.0, 1.0)]), Some( o(0.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(0.0, 1.0)]), Some(ro(0.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(0.0, 1.0)]), Some(ro(0.0, 2.0)));

    // Center Point connects.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(1.5, 1.5)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(1.5, 1.5)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(1.5, 1.5)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(1.5, 1.5)]), Some( o(1.0, 2.0)));

    // Left Point connects.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(1.0, 1.0)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(1.0, 1.0)]), Some(ro(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(1.0, 1.0)]), Some(ro(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(1.0, 1.0)]), Some(ro(1.0, 2.0)));

    // Right Point connects.
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  o(2.0, 2.0)]), Some( o(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), lo(2.0, 2.0)]), Some(lo(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0), ro(2.0, 2.0)]), Some(lo(1.0, 2.0)));
    assert_eq!(I::enclose(vec![o(1.0, 2.0),  c(2.0, 2.0)]), Some(lo(1.0, 2.0)));
}