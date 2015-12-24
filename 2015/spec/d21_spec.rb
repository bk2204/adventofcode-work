require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd21')

describe AdventOfCode::D21::Simulator do
  it 'should produce the correct result for a battle' do
    player = AdventOfCode::D21::Attacker.new(hit_points: 8, damage: 5, armor: 5)
    boss = AdventOfCode::D21::Attacker.new(hit_points: 12, damage: 7, armor: 2)
    sim = AdventOfCode::D21::Simulator.new(player, boss)
    expect(sim.winner).to eq :player
    expect(player.hit_points).to eq 2
    expect(boss.hit_points).to eq 0
  end

end

describe AdventOfCode::D21::EquipmentSet do
  it 'should apply equipment properly' do
    klass = AdventOfCode::D21::EquipmentSet
    choices = []
    choices << klass.weapons.find { |w| w[:name] == 'Dagger' }
    choices << klass.armor.find { |a| a[:name] == 'Splintmail' }
    choices << klass.rings.find { |r| r[:name] == 'Damage +1' }
    choices << klass.rings.find { |r| r[:name] == 'Defense +3' }
    set = klass.new(*choices)
    expect(set.damage).to eq 5
    expect(set.armor).to eq 6
    expect(set.cost).to eq (8 + 53 + 25 + 80)
    player = AdventOfCode::D21::Attacker.new(hit_points: 8)
    player.equipment = set
    expect(player.damage).to eq 5
    expect(player.armor).to eq 6
  end
end
