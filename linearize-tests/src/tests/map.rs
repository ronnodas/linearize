use {
    linearize::{static_copy_map, static_map, LinearizeExt, StaticCopyMap, StaticMap},
    std::{
        borrow::{Borrow, BorrowMut},
        cmp::Ordering,
        collections::HashMap,
        hash::{BuildHasher, RandomState},
        ops::{Deref, DerefMut},
    },
};

#[test]
fn map_equal_size() {
    let a = static_map! {
        false => 1,
        true => 2,
    };
    let x = a.clone().map_values(|v| v * 2);
    assert_eq!(x[false], 2);
    assert_eq!(x[true], 4);
    let x = a.map(|b, v| b as i32 + v);
    assert_eq!(x[false], 1);
    assert_eq!(x[true], 3);
}

#[test]
fn map_smaller_size() {
    let a = static_map! {
        false => 1u32,
        true => 2,
    };
    let x = a.map_values(|v| v as u8 * 2);
    assert_eq!(x[false], 2);
    assert_eq!(x[true], 4);
}

#[test]
fn map_larger_size() {
    let a = static_map! {
        false => 1u32,
        true => 2,
    };
    let x = a.map_values(|v| v as u64 * 2);
    assert_eq!(x[false], 2);
    assert_eq!(x[true], 4);
}

#[test]
fn map_padded() {
    struct X {
        _a: u16,
        _b: u8,
    }
    let a = static_map! {
        _ => X { _a: 0, _b: 0 }
    };
    let x = a.map_values(|_| [0u8; 4]);
    assert_eq!(x[false], [0; 4]);
}

#[test]
fn map_box() {
    let a = static_map! {
        _ => Box::new(0),
    };
    let x = a.map_values(|_| &0);
    assert_eq!(*x[false], 0);
}

#[test]
fn map_aligned() {
    #[repr(align(128))]
    struct Aligned(u8);
    let a = static_map! {
        _ => [0u8; 128],
    };
    let x = a.map_values(|_| Aligned(0));
    assert_eq!(x[false].0, 0);
}

#[test]
fn map_box_box() {
    let a = static_map! {
        _ => Box::new(0u8),
    };
    let x = a.map_values(|_| Box::new(0u64));
    assert_eq!(*x[false], 0);
}

#[test]
fn map_box_vec() {
    let a = static_map! {
        _ => Box::new(0u8),
    };
    let x = a.map_values(|_| vec![0u64]);
    assert_eq!(*x[false], [0]);
}

#[test]
fn map_vec_box() {
    let a = static_map! {
        _ => vec![0u8]
    };
    let x = a.map_values(|_| Box::new(0u64));
    assert_eq!(*x[false], 0);
}

#[test]
fn from_fn() {
    let map = StaticMap::from_fn(|k: bool| k as usize);
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
}

#[test]
fn from_ref() {
    let array = [0, 1];
    let map = StaticMap::from_ref(&array);
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
}

#[test]
fn from_mut() {
    let mut array = [0, 1];
    let map = StaticMap::from_mut(&mut array);
    map[false] = 1;
    map[true] = 0;
    assert_eq!(array, [1, 0]);
}

#[test]
fn into_copy() {
    let map: StaticMap<_, _> = static_map! {
        false => 0,
        true => 1,
    };
    let map: StaticCopyMap<_, _> = map.into_copy();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
}

#[test]
fn from_copy() {
    let map: StaticCopyMap<_, _> = static_copy_map! {
        false => 0,
        true => 1,
    };
    let map: StaticMap<_, _> = StaticMap::from_copy(map);
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
}

#[test]
fn as_copy() {
    let map: StaticMap<_, _> = static_map! {
        false => 0,
        true => 1,
    };
    let map: &StaticCopyMap<_, _> = map.as_copy();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
}

#[test]
fn as_copy_mut() {
    let mut map: StaticMap<_, _> = static_map! {
        false => 0,
        true => 1,
    };
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
    {
        let map: &mut StaticCopyMap<_, _> = map.as_copy_mut();
        map[false] = 1;
        map[true] = 0;
    }
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 0);
}

#[test]
fn each_ref() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let map: StaticCopyMap<_, &u8> = map.each_ref();
    assert_eq!(map[false], &0);
    assert_eq!(map[true], &1);
}

#[test]
fn each_mut() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
    {
        let mut map: StaticMap<_, &mut u8> = map.each_mut();
        *map[false] = 1;
        *map[true] = 0;
    }
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 0);
}

#[test]
fn map_values() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
    let map = map.map_values(|v| 3 * v);
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 3);
}

#[test]
fn clear() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 1);
    map.clear();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 0);
}

#[test]
fn keys() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let mut iter = map.keys();
    assert_eq!(iter.next(), Some(false));
    assert_eq!(iter.next(), Some(true));
    assert_eq!(iter.next(), None);
}

#[test]
fn values() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let mut iter = map.values();
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}

#[test]
fn values_mut() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let mut iter = map.values_mut();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    assert_eq!(iter.next(), None);
    assert_eq!(a, &mut 0);
    assert_eq!(b, &mut 1);
    *a = 2;
    *b = 3;
    assert_eq!(*map, [2, 3]);
}

#[test]
fn iter() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    {
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((false, &0)));
        assert_eq!(iter.next(), Some((true, &1)));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.next_back(), Some((true, &1)));
        assert_eq!(iter.next_back(), Some((false, &0)));
        assert_eq!(iter.next_back(), None);
    }
    {
        let iter = map.iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
    {
        let iter = map.iter();
        assert_eq!(iter.count(), 2);
    }
    {
        let iter = map.iter();
        assert_eq!(iter.last(), Some((true, &1)));
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.nth(0), Some((false, &0)));
        assert_eq!(iter.nth(0), Some((true, &1)));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.nth(1), Some((true, &1)));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.nth(2), None);
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.nth_back(0), Some((true, &1)));
        assert_eq!(iter.nth_back(0), Some((false, &0)));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.nth_back(1), Some((false, &0)));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.iter();
        assert_eq!(iter.nth_back(2), None);
    }
}

#[test]
fn iter_mut() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.next(), Some((false, &mut 0)));
        assert_eq!(iter.next(), Some((true, &mut 1)));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.next_back(), Some((true, &mut 1)));
        assert_eq!(iter.next_back(), Some((false, &mut 0)));
        assert_eq!(iter.next_back(), None);
    }
    {
        let iter = map.iter_mut();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
    {
        let iter = map.iter_mut();
        assert_eq!(iter.count(), 2);
    }
    {
        let iter = map.iter_mut();
        assert_eq!(iter.last(), Some((true, &mut 1)));
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.nth(0), Some((false, &mut 0)));
        assert_eq!(iter.nth(0), Some((true, &mut 1)));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.nth(1), Some((true, &mut 1)));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.nth(2), None);
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.nth_back(0), Some((true, &mut 1)));
        assert_eq!(iter.nth_back(0), Some((false, &mut 0)));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.nth_back(1), Some((false, &mut 0)));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.nth_back(2), None);
    }
}

#[test]
fn into_iter() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.next(), Some((false, 0)));
        assert_eq!(iter.next(), Some((true, 1)));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.next_back(), Some((true, 1)));
        assert_eq!(iter.next_back(), Some((false, 0)));
        assert_eq!(iter.next_back(), None);
    }
    {
        let iter = map.clone().into_iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
    {
        let iter = map.clone().into_iter();
        assert_eq!(iter.count(), 2);
    }
    {
        let iter = map.clone().into_iter();
        assert_eq!(iter.last(), Some((true, 1)));
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.nth(0), Some((false, 0)));
        assert_eq!(iter.nth(0), Some((true, 1)));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.nth(1), Some((true, 1)));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.nth(2), None);
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.nth_back(0), Some((true, 1)));
        assert_eq!(iter.nth_back(0), Some((false, 0)));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.nth_back(1), Some((false, 0)));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.nth_back(2), None);
    }
}

#[test]
fn into_values() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), None);
    }
    {
        let iter = map.clone().into_values();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
    {
        let iter = map.clone().into_values();
        assert_eq!(iter.count(), 2);
    }
    {
        let iter = map.clone().into_values();
        assert_eq!(iter.last(), Some(1));
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.nth(0), Some(0));
        assert_eq!(iter.nth(0), Some(1));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.nth(1), Some(1));
        assert_eq!(iter.nth(0), None);
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.nth(2), None);
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.nth_back(0), Some(1));
        assert_eq!(iter.nth_back(0), Some(0));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.nth_back(1), Some(0));
        assert_eq!(iter.nth_back(0), None);
    }
    {
        let mut iter = map.clone().into_values();
        assert_eq!(iter.nth_back(2), None);
    }
}

#[test]
fn deref() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    assert_eq!(map.deref(), &[0, 1]);
    assert_eq!(map.deref_mut(), &mut [0, 1]);
}

#[test]
fn from_iter() {
    let map: StaticMap<_, u8> = [(false, 1)].into_iter().collect();
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 0);
}

#[test]
fn index() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    map[false] = 3;
    map[true] = 4;
    assert_eq!(map[false], 3);
    assert_eq!(map[true], 4);
    map[&false] = 5;
    map[&true] = 6;
    assert_eq!(map[&false], 5);
    assert_eq!(map[&true], 6);
    map[false.linearized()] = 7;
    map[true.linearized()] = 8;
    assert_eq!(map[false.linearized()], 7);
    assert_eq!(map[true.linearized()], 8);
}

#[test]
fn as_ref() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let mut copy_map = map.clone().into_copy();
    assert_eq!(AsRef::<[u8]>::as_ref(&map), &[0, 1]);
    assert_eq!(AsMut::<[u8]>::as_mut(&mut map), &mut [0, 1]);
    assert_eq!(AsRef::<StaticCopyMap<_, _>>::as_ref(&map), &copy_map);
    assert_eq!(
        AsMut::<StaticCopyMap<_, _>>::as_mut(&mut map),
        &mut copy_map
    );
}

#[test]
fn borrow() {
    let mut map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let mut copy_map = map.clone().into_copy();
    assert_eq!(Borrow::<[u8]>::borrow(&map), &[0, 1]);
    assert_eq!(BorrowMut::<[u8]>::borrow_mut(&mut map), &mut [0, 1]);
    assert_eq!(Borrow::<StaticCopyMap<_, _>>::borrow(&map), &copy_map);
    assert_eq!(
        BorrowMut::<StaticCopyMap<_, _>>::borrow_mut(&mut map),
        &mut copy_map
    );
}

#[test]
fn debug() {
    let map1: StaticMap<_, u8> = static_map! {
        () => 0,
    };
    let mut map2 = HashMap::new();
    map2.insert((), 0);
    assert_eq!(format!("{:?}", map1), format!("{:?}", map2));
}

#[test]
fn default() {
    #[derive(Default, PartialEq, Debug)]
    struct X;
    let map = StaticMap::<_, X>::default();
    assert_eq!(map[false], X);
    assert_eq!(map[true], X);
}

#[test]
fn hash() {
    let map: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let random = RandomState::new();
    assert_eq!(random.hash_one(map), random.hash_one([0u8, 1]));
}

#[test]
fn ord() {
    let map1: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let map2: StaticMap<_, u8> = static_map! {
        false => 1,
        true => 1,
    };
    let map3: StaticMap<_, u8> = static_map! {
        false => 2,
        true => 1,
    };
    assert_eq!(map1.cmp(&map2), Ordering::Less);
    assert_eq!(map1.cmp(&map1), Ordering::Equal);
    assert_eq!(map2.cmp(&map1), Ordering::Greater);
    assert_eq!(map1.clone().max(map2.clone()), map2);
    assert_eq!(map2.clone().max(map1.clone()), map2);
    assert_eq!(map1.clone().min(map2.clone()), map1);
    assert_eq!(map2.clone().min(map1.clone()), map1);
    assert_eq!(map1.clone().clamp(map1.clone(), map1.clone()), map1);
    assert_eq!(map1.clone().clamp(map1.clone(), map2.clone()), map1);
    assert_eq!(map1.clone().clamp(map1.clone(), map3.clone()), map1);
    assert_eq!(map1.clone().clamp(map2.clone(), map2.clone()), map2);
    assert_eq!(map1.clone().clamp(map2.clone(), map3.clone()), map2);
    assert_eq!(map1.clone().clamp(map3.clone(), map3.clone()), map3);
    assert_eq!(map2.clone().clamp(map1.clone(), map1.clone()), map1);
    assert_eq!(map2.clone().clamp(map1.clone(), map2.clone()), map2);
    assert_eq!(map2.clone().clamp(map1.clone(), map3.clone()), map2);
    assert_eq!(map2.clone().clamp(map2.clone(), map2.clone()), map2);
    assert_eq!(map2.clone().clamp(map2.clone(), map3.clone()), map2);
    assert_eq!(map2.clone().clamp(map3.clone(), map3.clone()), map3);
    assert_eq!(map3.clone().clamp(map1.clone(), map1.clone()), map1);
    assert_eq!(map3.clone().clamp(map1.clone(), map2.clone()), map2);
    assert_eq!(map3.clone().clamp(map1.clone(), map3.clone()), map3);
    assert_eq!(map3.clone().clamp(map2.clone(), map2.clone()), map2);
    assert_eq!(map3.clone().clamp(map2.clone(), map3.clone()), map3);
    assert_eq!(map3.clone().clamp(map3.clone(), map3.clone()), map3);
}

#[test]
fn partial_eq() {
    let map1: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let map2: StaticMap<_, u8> = static_map! {
        false => 1,
        true => 1,
    };
    assert_eq!(map1.eq(&map1), true);
    assert_eq!(map1.eq(&map2), false);
    assert_eq!(map2.eq(&map1), false);
    assert_eq!(map2.eq(&map2), true);
}

#[test]
fn partial_ord() {
    let map1: StaticMap<_, u8> = static_map! {
        false => 0,
        true => 1,
    };
    let map2: StaticMap<_, u8> = static_map! {
        false => 1,
        true => 1,
    };
    assert_eq!(map1.partial_cmp(&map2), Some(Ordering::Less));
    assert_eq!(map1.partial_cmp(&map1), Some(Ordering::Equal));
    assert_eq!(map2.partial_cmp(&map1), Some(Ordering::Greater));

    let map1: StaticMap<_, f64> = static_map! {
        false => f64::NAN,
        true => 1.0,
    };
    let map2: StaticMap<_, f64> = static_map! {
        false => f64::NAN,
        true => 1.0,
    };
    assert_eq!(map1.partial_cmp(&map2), None);
    assert_eq!(map1.partial_cmp(&map1), None);
    assert_eq!(map2.partial_cmp(&map1), None);
}

#[test]
fn try_from() {
    let mut map = static_map! {
        false => 0,
        true => 1,
    };

    let slice: &[u8] = &[0, 1];
    assert_eq!(<&StaticMap<bool, _>>::try_from(slice).unwrap(), &map);
    assert_eq!(<StaticMap<bool, _>>::try_from(slice).unwrap(), map);
    let slice: &mut [u8] = &mut [0, 1];
    assert_eq!(
        <&mut StaticMap<bool, _>>::try_from(slice).unwrap(),
        &mut map
    );
    let slice: &mut [u8] = &mut [0, 1];
    assert_eq!(<StaticMap<bool, _>>::try_from(slice).unwrap(), map);
    let vec = vec![0, 1];
    assert_eq!(<StaticMap<bool, _>>::try_from(vec).unwrap(), map);

    let slice: &[u8] = &[0];
    assert!(<&StaticMap<bool, _>>::try_from(slice).is_err());
    assert!(<StaticMap<bool, _>>::try_from(slice).is_err());
    let slice: &mut [u8] = &mut [0];
    assert!(<&mut StaticMap<bool, _>>::try_from(slice).is_err());
    let slice: &mut [u8] = &mut [0];
    assert!(<StaticMap<bool, _>>::try_from(slice).is_err());
    let vec = vec![0];
    assert!(<StaticMap<bool, _>>::try_from(vec).is_err());

    let slice: &[u8] = &[0, 1, 2];
    assert!(<&StaticMap<bool, _>>::try_from(slice).is_err());
    assert!(<StaticMap<bool, _>>::try_from(slice).is_err());
    let slice: &mut [u8] = &mut [0, 1, 2];
    assert!(<&mut StaticMap<bool, _>>::try_from(slice).is_err());
    let slice: &mut [u8] = &mut [0, 1, 2];
    assert!(<StaticMap<bool, _>>::try_from(slice).is_err());
    let vec = vec![0, 1, 2];
    assert!(<StaticMap<bool, _>>::try_from(vec).is_err());
}

#[test]
fn from_iterator() {
    let array: [(bool, u8); 0] = [];
    let map: StaticMap<bool, u8> = array.into_iter().collect();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 0);
    let array: [(&bool, u8); 0] = [];
    let map: StaticMap<bool, u8> = array.into_iter().collect();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 0);
    let map: StaticMap<bool, u8> = [(false, 1)].into_iter().collect();
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 0);
    let map: StaticMap<bool, u8> = [(&false, 1)].into_iter().collect();
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 0);
    let map: StaticMap<bool, u8> = [(true, 2)].into_iter().collect();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 2);
    let map: StaticMap<bool, u8> = [(&true, 2)].into_iter().collect();
    assert_eq!(map[false], 0);
    assert_eq!(map[true], 2);
    let map: StaticMap<bool, u8> = [(false, 1), (true, 2)].into_iter().collect();
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 2);
    let map: StaticMap<bool, u8> = [(&false, 1), (&true, 2)].into_iter().collect();
    assert_eq!(map[false], 1);
    assert_eq!(map[true], 2);
}
