require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd17')

describe AdventOfCode::D17::ContainerSet do
  it 'should produce the right number of combinations' do
    s = AdventOfCode::D17::ContainerSet.new([20, 15, 10, 5, 5], 25)
    expect(s.longest_set).to eq 3
  end

  it 'should produce the right number of combinations' do
    s = AdventOfCode::D17::ContainerSet.new([20, 15, 10, 5, 5], 25)
    expect(s.count).to eq 4
  end
end
