require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd8')

describe AdventOfCode::D8::Processor do
  it 'should produce the right values for each wire' do
    p = AdventOfCode::D8::Processor.new
    data = [
      ['""', 2, 0],
      ['"abc"', 5, 3],
      ['"aaa\"aaa"', 10, 7],
      ['"\x27"', 6, 1]
    ]
    data.each do |(s, code, text)|
      expect(p.as_code(s)).to eq code
      expect(p.as_text(s)).to eq text
    end
    count = data.map { |a| a[0] }.reduce(0) do |memo, s|
      memo + p.as_code(s) - p.as_text(s)
    end
    expect(count).to eq 12
  end
end
