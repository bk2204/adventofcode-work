require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd8')

describe AdventOfCode::D8::Processor do
  data = [
    ['""', 2, 0, 6],
    ['"abc"', 5, 3, 9],
    ['"aaa\"aaa"', 10, 7, 16],
    ['"\x27"', 6, 1, 11]
  ]

  it 'should produce the right values for each string' do
    p = AdventOfCode::D8::Processor.new
    data.each do |(s, code, text, encoded)|
      expect(p.as_code(s)).to eq code
      expect(p.as_text(s)).to eq text
      expect(p.encode(s).length).to eq encoded
    end
  end

  it 'should produce the right total difference' do
    p = AdventOfCode::D8::Processor.new
    count = data.map { |a| a[0] }.reduce(0) do |memo, s|
      memo + p.as_code(s) - p.as_text(s)
    end
    expect(count).to eq 12
  end

  it 'should produce the right total difference for encoded strings' do
    p = AdventOfCode::D8::Processor.new
    count = data.map { |a| a[0] }.reduce(0) do |memo, s|
      memo + p.encode(s).length - p.as_code(s)
    end
    expect(count).to eq 19
  end
end
