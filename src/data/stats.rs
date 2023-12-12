use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stat
{
	Hp,
	Atk,
	Def,
	SpAtk,
	SpDef,
	Spe,
}
impl FromStr for Stat
{
	type Err = ParseStatError;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s.to_lowercase().as_str()
		{
			"hp" => Ok(Self::Hp),
			"atk" | "attack" => Ok(Self::Atk),
			"def" | "defense" | "defence" => Ok(Self::Def),
			"spa" | "spatk" | "special attack" | "sp attack" => Ok(Self::SpAtk),
			"spd" | "spdef" | "special defense" | "sp defense" | "special defence"
			| "sp defence" => Ok(Self::SpDef),
			"spe" | "speed" => Ok(Self::Spe),
			_ => Err(ParseStatError),
		}
	}
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Nature
{
	pub increased: Stat,
	pub decreased: Stat,
}
impl Default for Nature
{
	fn default() -> Self
	{
		Self {
			increased: Stat::Spe,
			decreased: Stat::Spe,
		}
	}
}
impl Nature
{
	pub fn multiplier(&self, stat: Stat) -> f32
	{
		// is there a more idomatic way to do this? maybe! -morgan 2023-12-11
		if self.increased == self.decreased
		{
			1.0
		}
		else if stat == self.increased
		{
			1.1
		}
		else if stat == self.decreased
		{
			0.9
		}
		else
		{
			1.0
		}
	}
}

#[derive(Debug)]
pub struct ParseStatError;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StatBlock
{
	pub hp: i32,
	pub atk: i32,
	pub def: i32,
	pub spatk: i32,
	pub spdef: i32,
	pub spe: i32,
}
impl std::ops::Index<Stat> for StatBlock
{
	type Output = i32;
	fn index(&self, index: Stat) -> &Self::Output
	{
		match index
		{
			Stat::Hp => &self.hp,
			Stat::Atk => &self.atk,
			Stat::Def => &self.def,
			Stat::SpAtk => &self.spatk,
			Stat::SpDef => &self.spdef,
			Stat::Spe => &self.spe,
		}
	}
}
impl StatBlock
{
	pub fn all(val: i32) -> Self
	{
		Self {
			hp: val,
			atk: val,
			def: val,
			spatk: val,
			spdef: val,
			spe: val,
		}
	}

	pub fn for_each_stat<F: Fn(Stat) -> i32>(generator: F) -> Self
	{
		Self {
			hp: generator(Stat::Hp),
			atk: generator(Stat::Atk),
			def: generator(Stat::Def),
			spatk: generator(Stat::SpAtk),
			spdef: generator(Stat::SpDef),
			spe: generator(Stat::Spe),
		}
	}
}

pub fn effort_bonus(effort_level: i32, pokemon_level: u8, base_stat: i32) -> Option<i32>
{
	Some(
		(((base_stat as f32).sqrt() * effort_multiplier(effort_level)? as f32
			+ pokemon_level as f32)
			/ 2.5)
			.round() as i32,
	)
}

fn effort_multiplier(effort_level: i32) -> Option<i32>
{
	// 0, 2, 3, 4, 7, 8, 9, 14, 15, 16, 25
	match effort_level
	{
		0 => Some(0),
		1 => Some(2),
		2 => Some(3),
		3 => Some(4),
		4 => Some(7),
		5 => Some(8),
		6 => Some(9),
		7 => Some(14),
		8 => Some(15),
		9 => Some(16),
		10 => Some(25),
		_ => None,
	}
}
