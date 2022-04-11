#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($e:expr),+ $(,)*) => {{
        let mut vs = avec![];
        $(vs.push($e);)+
        vs
    }};
    ($e: expr; $count:expr) => {{
        let mut vs = avec![];
        let x = $e;
        for _ in 0..$count {
            vs.push(x.clone());
        }
        vs
    }};
}

#[test]
fn test_expand() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn test_single() {
    let x: Vec<u32> = avec![41];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 41);
}

#[test]
fn test_double() {
    let x: Vec<u32> = avec![41, 42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 41);
    assert_eq!(x[1], 42);
}

#[test]
fn test_clone() {
    let x: Vec<u32> = avec![41; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 41);
    assert_eq!(x[1], 41);
}

#[test]
fn test_clone_nonliteral() {
    let mut y = Some(41);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 41);
    assert_eq!(x[1], 41);
}
