require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd14')

describe AdventOfCode::D14::Reindeer do
  it 'should compute the correct values for Comet' do
    line = \
      'Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds'
    [
      AdventOfCode::D14::Reindeer.new('Comet', 14, 10, 127),
      AdventOfCode::D14::Reindeer.parse(line)
    ].each do |r|
      expect(r.distance(5)).to eq 70
      expect(r.distance(10)).to eq 140
      expect(r.distance(50)).to eq 140
      expect(r.distance(137)).to eq 140
      expect(r.distance(138)).to eq 154
    end
  end

  it 'should compute the correct value for 1000 seconds' do
    [
      ['Comet', 14, 10, 127, 1120],
      ['Dancer', 16, 11, 162, 1056]
    ].each do |(name, speed, time, slack_time, distance)|
      r = AdventOfCode::D14::Reindeer.new(name, speed, time, slack_time)
      expect(r.distance(1000)).to eq distance
    end
  end
end
