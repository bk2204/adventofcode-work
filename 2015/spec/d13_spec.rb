require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd13')

describe AdventOfCode::D13::Seater do
  it 'should compute the correct value for certain strings' do
    p = AdventOfCode::D13::Seater.new
    rules = <<-EOM.gsub(/^\s+/, '')
    Alice would gain 54 happiness units by sitting next to Bob.
    Alice would lose 79 happiness units by sitting next to Carol.
    Alice would lose 2 happiness units by sitting next to David.
    Bob would gain 83 happiness units by sitting next to Alice.
    Bob would lose 7 happiness units by sitting next to Carol.
    Bob would lose 63 happiness units by sitting next to David.
    Carol would lose 62 happiness units by sitting next to Alice.
    Carol would gain 60 happiness units by sitting next to Bob.
    Carol would gain 55 happiness units by sitting next to David.
    David would gain 46 happiness units by sitting next to Alice.
    David would lose 7 happiness units by sitting next to Bob.
    David would gain 41 happiness units by sitting next to Carol.
    EOM
    rules.each_line { |line| p.parse(line) }
    expect(p.optimal_happiness).to eq 330
  end
end
