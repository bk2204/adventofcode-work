require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd16')

describe AdventOfCode::D16::Sue do
  it 'should match when attributes are the same' do
    s1 = AdventOfCode::D16::Sue.new(1, attr: 1, attr2: 2)
    s2 = AdventOfCode::D16::Sue.new(1, attr: 1, attr2: 2)
    expect(s1 === s2).to eq true
  end

  it 'should match when existing attributes are the same' do
    s1 = AdventOfCode::D16::Sue.new(1, attr: 1, attr2: 2)
    s2 = AdventOfCode::D16::Sue.new(1, attr: 1)
    expect(s1 === s2).to eq true
  end

  it 'should not match when existing attributes are different' do
    s1 = AdventOfCode::D16::Sue.new(1, attr: 1, attr2: 2)
    s2 = AdventOfCode::D16::Sue.new(1, attr: 2)
    expect(s1 === s2).to eq false
  end

  it 'should not match when attributes are different' do
    s1 = AdventOfCode::D16::Sue.new(1, attr: 1, attr2: 2)
    s2 = AdventOfCode::D16::Sue.new(1, attr: 2, attr2: 2)
    expect(s1 === s2).to eq false
  end

  it 'should parse attributes correctly' do
    line = 'Sue 1: goldfish: 9, cars: 0, samoyeds: 9'
    sue = AdventOfCode::D16::Sue.parse(line)
    expected = {
      goldfish: 9,
      cars: 0,
      samoyeds: 9
    }
    expect(sue.num).to eq 1
    expect(sue.attributes).to eq expected
  end
end
