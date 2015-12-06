require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd3')

describe AdventOfCode::D3::Processor do
  def process(s)
    p = AdventOfCode::D3::Processor.new
    p.process(s)
    p.locations
  end

  it 'should process testcases correctly' do
    expect(process('>').length).to eq 2
    expect(process('^>v<').length).to eq 4
    expect(process('^v^v^v^v^v').length).to eq 2
  end
end
