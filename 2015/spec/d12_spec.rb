require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd12')

describe AdventOfCode::D12::Extractor do
  it 'should compute the correct value for certain strings' do
    p = AdventOfCode::D12::Extractor.new
    [
      ['[1,2,3]', 6],
      ['{"a":2,"b":4}', 6],
      ['[[[3]]', 3],
      ['{"a":{"b":4},"c":-1}', 3],
      ['{"a":[-1,1]}', 0],
      ['[-1,{"a":1}]', 0],
      ['[]', 0],
      ['{}', 0]
    ].each do |(s, total)|
      expect(p.sum(s)).to eq total
    end
  end
end
