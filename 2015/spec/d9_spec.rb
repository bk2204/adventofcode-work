require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd9')

describe AdventOfCode::D9::GraphTraverser do
  it 'should produce the right values for a sample path' do
    p = AdventOfCode::D9::GraphTraverser.new
    text = <<-EOM.gsub(/^\s+/, '')
    London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141
    EOM
    text.each_line { |line| p.parse_path(line) }
    expect(p.find_shortest_path).to eq 605
    expect(p.find_path(:<)).to eq 605
    expect(p.find_path(:>)).to eq 982
  end
end
