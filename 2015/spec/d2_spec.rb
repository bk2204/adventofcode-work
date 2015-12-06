require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd2')

describe AdventOfCode::D2::Processor do
  it 'should compute testcases correctly' do
    [
      [2, 3, 4, 58],
      [1, 1, 10, 43]
    ].each do |(d1, d2, d3, total)|
      p = AdventOfCode::D2::Processor.new(d1, d2, d3)
      expect(p.total).to eq total
    end
  end

  it 'automatically sort items' do
    [
      [3, 4, 2, 58],
      [1, 10, 1, 43]
    ].each do |(d1, d2, d3, total)|
      p = AdventOfCode::D2::Processor.new(d1, d2, d3)
      expect(p.total).to eq total
    end
  end
end
