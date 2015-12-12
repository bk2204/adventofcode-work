require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd7')

describe AdventOfCode::D7::Circuit do
  it 'should produce the right values for each wire' do
    c = AdventOfCode::D7::Circuit.new
    rules = <<-EOM.gsub(/^\s+/m, '')
    123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> i
    EOM
    rules.each_line { |line| c.process(line) }
    expect(c.signal(:d)).to eq 72
    expect(c.signal(:e)).to eq 507
    expect(c.signal(:f)).to eq 492
    expect(c.signal(:g)).to eq 114
    expect(c.signal(:h)).to eq 65412
    expect(c.signal(:i)).to eq 65079
    expect(c.signal(:x)).to eq 123
    expect(c.signal(:y)).to eq 456
  end
end
