require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd20')

describe AdventOfCode::D20::PresentDeliverer do
  it 'should produce the proper number of presents' do
    pd = AdventOfCode::D20::PresentDeliverer.new
    expect(pd.take(9)).to eq [10, 30, 40, 70, 60, 120, 80, 150, 130]
  end
end
