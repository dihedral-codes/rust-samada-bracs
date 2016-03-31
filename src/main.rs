
fn get(b: u64, i: u8) -> u8 {
  if (b >> i) & 1u64 == 0 {
    return 0;
  } else {
    return 1
  }
}

fn set(b: &mut u64, i: u8, val: u8) {

  if val == 0 {
    *b &= !(1u64 << i);
  } else {
    *b |= 1u64 << i;
  }

}

fn print(x: u64, n: u8) {
  println!("x: {x:0>0$b}", n as usize, x=x);
}


fn checkrev(b: u64, t: u8, i:u8) -> isize {

  for j in (i + 1)..((t + 3) / 2) {
    if get(b, j) < get(b, t - j + 1) {
      return 0;
    }
    
    if get(b, j) < get(b, t - j + 1) {
      return -1;
    }
  }

  return 1;

}


fn generate(b: &mut u64, n:u8, t:i8, p:i8, mut r:i8, mut u:i8, mut v:i8, mut rs: i8) {

  if t - 1 > ((n as i8 - r) / 2) + r {
    if get(*b, (t - 1) as u8) > get(*b, n - t as u8 + 2 + r as u8) {
      rs = 0;
    } else if get(*b, t as u8 - 1) < get(*b, n - t as u8 + 2 + r as u8) {
      rs = 1;
    }
  }

  if t as u8 > n {
    if rs == 0 && n % p as u8 == 0 {
      print(*b, n);
    }
  } else {
    let x = get(*b, (t - p) as u8);
    set(b, t as u8, x);

    if get(*b, t as u8) == get(*b, 1) {
      v += 1;
    } else {
      v = 0;
    }

    if u == -1 && get(*b, t as u8 - 1) != get(*b, 1) {
      u = t - 2;
      r = t - 2;
    }

    if u != -1 && t as u8 == n && get(*b, n) == get(*b, 1) {

    } else if u == v {

      let rev = checkrev(*b, t as u8, u as u8);
      if rev == 0 {
        generate(b, n, t + 1, p, r, u, v, rs);
      }
      if rev == 1 {
        generate(b, n, t + 1, p, t, u, v, 0);
      }

    } else {
      generate(b, n, t + 1, p, r, u, v, rs); 
    }

    let j = get(*b, (t - p) as u8) + 1;
    while j <= 1 {
      set(b, t as u8, j);
      generate(b, n, t + 1, t, r, u, 0, rs);
    }

  }

}




fn main() {
  let n:u8 = 16;
  let total = 0;

  let mut b: u64 = 0b101011;

  set(&mut b, 2, 1);

  println!("n: {:02}", n);
  println!("total: {:02}", total);
  print(b, n);

  println!("{}", checkrev(b, 1, 2));

  generate(&mut b,n,1,1,1,-1,0,0);

}
