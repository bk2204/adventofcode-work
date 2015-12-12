require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd6')

describe AdventOfCode::D6::LightArray do
  it 'should turn on lights correctly' do
    a = AdventOfCode::D6::LightArray.new
    a.process('turn on 0,0 through 999,999')
    expect(a.on?(0, 0)).to eq true
    expect(a.on?(999, 999)).to eq true
    expect(a.on?(0, 999)).to eq true
    expect(a.on?(999, 0)).to eq true
    expect(a.on?(5, 3)).to eq true
    expect(a.on?(100, 100)).to eq true
  end

  it 'should count on lights correctly' do
    a = AdventOfCode::D6::LightArray.new
    a.process('turn on 0,0 through 999,999')
    expect(a.count).to eq 1000 * 1000
    a.process('toggle 0,0 through 999,0')
    expect(a.count).to eq 999 * 1000
    a.process('turn off 499,499 through 500,500')
    expect(a.count).to eq 999 * 1000 - 4
  end

  it 'should count brightness correctly' do
    a = AdventOfCode::D6::BrightnessArray.new
    a.process('turn on 0,0 through 0,0')
    expect(a.count).to eq 1
    a = AdventOfCode::D6::BrightnessArray.new
    a.process('toggle 0,0 through 999,999')
    expect(a.count).to eq 2000000
  end
end
