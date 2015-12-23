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
end
