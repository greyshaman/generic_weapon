use std::fmt::Debug;

use generic_weapon::{
    assoc_type::Shoot, shell::{arrow::Arrow, bullet::Bullet, rocket::Rocket}, weapon::{
        blaster::Blaster, crossbow::Crossbow, pistol::Pistol, rocket_launcher::RocketLauncher,
    }
};

fn fire_and_forget<W>(weapon: &mut W, target: &str)
where
    W: Shoot,
{
    match weapon.shoot(target) {
        Ok((ammo, byproduct)) => {
            println!(" Снаряд: {:?}", std::any::type_name_of_val(&ammo));
            println!(
                " Побочный продукт: {:?}",
                std::any::type_name_of_val(&byproduct)
            );
        }
        Err(e) => eprintln!("  Ошибка: {:?}", e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pistol = Pistol::new(9.0);
    fire_and_forget(&mut pistol, "Мишень");
    pistol.reload(Bullet::new(9.0))?;
    fire_and_forget(&mut pistol, "Мишень");
    println!();

    let mut crossbow = Crossbow::new(150.0);
    fire_and_forget(&mut crossbow, "Яблоко");
    crossbow.reload(Arrow::new(300.0, "Iron"))?;
    // crossbow.reload(Bullet::new(9.0))?;
    fire_and_forget(&mut crossbow, "Яблоко");
    println!();

    let mut blaster = Blaster::new();
    fire_and_forget(&mut blaster, "Дрон");
    println!();


    let mut rocket_launcher = RocketLauncher::new();
    fire_and_forget(&mut rocket_launcher, "Метеорит");
    rocket_launcher.reload(Rocket::new("Antimatter", 25000.0))?;
    fire_and_forget(&mut rocket_launcher, "Метеорит");

    Ok(())
}
