require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd2')

describe AdventOfCode::D2::Present do
  it 'should compute paper testcases correctly' do
    [
      [2, 3, 4, 58],
      [1, 1, 10, 43]
    ].each do |(d1, d2, d3, total)|
      p = AdventOfCode::D2::Present.new(d1, d2, d3)
      expect(p.total_paper).to eq total
    end
  end

  it 'should automatically sort items' do
    [
      [3, 4, 2, 58],
      [1, 10, 1, 43]
    ].each do |(d1, d2, d3, total)|
      p = AdventOfCode::D2::Present.new(d1, d2, d3)
      expect(p.total_paper).to eq total
    end
  end

  it 'should compute ribbon testcases correctly' do
    [
      [3, 4, 2, 10, 24, 34],
      [1, 10, 1, 4, 10, 14]
    ].each do |(d1, d2, d3, dist, vol, total)|
      p = AdventOfCode::D2::Present.new(d1, d2, d3)
      expect(p.shortest_distance).to eq dist
      expect(p.volume).to eq vol
      expect(p.total_ribbon).to eq total
    end
  end
end
