require 'rspec'
require 'set'
load File.join(File.dirname(__FILE__), '..', 'd19')

describe AdventOfCode::D19::Replacer do
  rules = <<-EOM.gsub(/^\s+/, '')
  H => HO
  H => OH
  O => HH
  EOM

  it 'should produce the proper sequences' do
    rep = AdventOfCode::D19::Replacer.parse(rules)
    results = rep.combinations('HOH')
    expect(results).to eq Set.new(%w(HOOH HOHO OHOH HHHH))
  end

  it 'should count states properly' do
    rep = AdventOfCode::D19::Replacer.parse(rules)
    results = rep.combinations('HOHOHO')
    expect(results.length).to eq 7
  end

  it 'should count steps properly' do
    new_rules = <<-EOM.gsub(/^\s+/, '')
    e => H
    e => O
    H => HO
    H => OH
    O => HH
    EOM
    rep = AdventOfCode::D19::Replacer.parse(new_rules)
    expect(rep.derive('HOH')).to eq 3
    expect(rep.derive('HOHOHO')).to eq 6
  end
end
