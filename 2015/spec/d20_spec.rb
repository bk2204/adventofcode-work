require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd20')

describe AdventOfCode::D20::PresentDeliverer do
  it 'should produce the proper number of presents' do
    pd = AdventOfCode::D20::PresentDeliverer.new(10)
    expect(pd.take(9)).to eq [10, 30, 40, 70, 60, 120, 80, 150, 130]
  end

  it 'should produce the proper number of presents with a limit' do
    pd = AdventOfCode::D20::PresentDeliverer.new(10, 3)
    expect(pd.take(8)).to eq [10, 30, 40, 60, 50, 110, 70, 120]
  end
end
