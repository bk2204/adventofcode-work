require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd18')

describe AdventOfCode::D18::LightArray do
  example = <<-EOM.gsub(/^\s+/, '')
  .#.#.#
  ...##.
  #....#
  ..#...
  #.#..#
  ####..
  EOM

  it 'should produce the right number of lights set' do
    la = AdventOfCode::D18::LightArray.new(6)
    la.load_state(example)
    expect(la.count).to eq 15
  end

  it 'should progress states properly' do
    la = AdventOfCode::D18::LightArray.new(6)
    la.load_state(example)
    la.next!
    expect(la.count).to eq 11
    la.next!
    expect(la.count).to eq 8
    la.next!
    expect(la.count).to eq 4
    la.next!
    expect(la.count).to eq 4
  end
end
