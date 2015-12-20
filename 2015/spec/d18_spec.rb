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

  it 'should produce the right number of lights in stuck mode' do
    la = AdventOfCode::D18::LightArray.new(6, true)
    la.load_state(example)
    expect(la.count).to eq 17
  end

  it 'should progress states properly' do
    la = AdventOfCode::D18::LightArray.new(6, true)
    la.load_state(example)
    expect(la.to_s).to eq <<-EOM.gsub(/^\s+/, '')
    ##.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####.#
    EOM
    expect(la.count).to eq 17
    la.next!
    expect(la.to_s).to eq <<-EOM.gsub(/^\s+/, '')
    #.##.#
    ####.#
    ...##.
    ......
    #...#.
    #.####
    EOM
    expect(la.count).to eq 18
    la.next!
    expect(la.count).to eq 18
    la.next!
    expect(la.count).to eq 18
    la.next!
    expect(la.count).to eq 14
    la.next!
    expect(la.count).to eq 17
  end
end
