require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd15')

describe AdventOfCode::D15::Optimizer do
  ingred = [
    AdventOfCode::D15::Ingredient.new('Butterscotch', -1, -2, 6, 3, 8),
    AdventOfCode::D15::Ingredient.new('Cinnamon', 2, 3, -2, -1, 3)
  ]

  it 'should compute a cookie score' do
    values = {
      ingred[0] => 44,
      ingred[1] => 56,
    }
    cookie = AdventOfCode::D15::Cookie.new(values)
    expect(cookie.score).to eq 62842880
  end

  it 'should be able to add cookies' do
    cookie = AdventOfCode::D15::Cookie.new({ ingred[0] => 44})
    cookie2 = AdventOfCode::D15::Cookie.new({ ingred[1] => 56})
    expect((cookie + cookie2).score).to eq 62842880
  end

  it 'should compute space correctly' do
    cookie = AdventOfCode::D15::Cookie.new({ ingred[0] => 44})
    expect(cookie.space).to eq 56
  end

  it 'should compute calories correctly' do
    values = {
      ingred[0] => 40,
      ingred[1] => 60
    }
    cookie = AdventOfCode::D15::Cookie.new(values)
    expect(cookie.calories).to eq 500
  end

  it 'should compute the correct values for two ingredients' do
    opt = AdventOfCode::D15::Optimizer.new(ingred)
    expect(opt.optimize).to eq 62842880
  end

  it 'should compute the correct values for two ingredients and 500 calories' do
    opt = AdventOfCode::D15::Optimizer.new(ingred, 500)
    expect(opt.optimize).to eq 57600000
  end
end
