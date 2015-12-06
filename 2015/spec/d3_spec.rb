require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd3')

describe AdventOfCode::D3::Processor do
  def process(nprocs, s)
    p = AdventOfCode::D3::Demuxer.new(nprocs)
    p.process(s)
    p.locations
  end

  it 'should process single-Santa testcases correctly' do
    expect(process(1, '>').length).to eq 2
    expect(process(1, '^>v<').length).to eq 4
    expect(process(1, '^v^v^v^v^v').length).to eq 2
  end

  it 'should process multiple-Santa testcases correctly' do
    expect(process(2, '^v').length).to eq 3
    expect(process(2, '^>v<').length).to eq 3
    expect(process(2, '^v^v^v^v^v').length).to eq 11
  end
end
