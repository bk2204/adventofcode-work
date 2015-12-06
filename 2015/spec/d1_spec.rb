require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd1')

describe AdventOfCode::D1P1::Processor do
  def process(s)
    p = AdventOfCode::D1P1::Processor.new
    p.process(s)
    p.floor
  end

  it 'should process parentheses correctly' do
    expect(process('(())')).to eq 0
    expect(process('()()')).to eq 0
    expect(process('(((')).to eq 3
    expect(process('(()(()(')).to eq 3
    expect(process('))(((((')).to eq 3
  end

  it 'should process negative numbers correctly' do
    expect(process('())')).to eq (-1)
    expect(process('))(')).to eq (-1)
    expect(process(')))')).to eq (-3)
    expect(process(')())())')).to eq (-3)
  end

  it 'should process multiple calls correctly' do
    p = AdventOfCode::D1P1::Processor.new
    p.process('())')
    p.process(')))')
    expect(p.floor).to eq (-4)
  end

  it 'should give proper results for the callback with short data' do
    cb_data = []
    cb = ->(floor, pos) { cb_data << [floor, pos] }
    p = AdventOfCode::D1P1::Processor.new(cb)
    p.process(')')
    expect(cb_data).to eq [[-1, 1]]
  end

  it 'should give proper results for the callback with longer data' do
    cb_data = []
    cb = ->(floor, pos) { cb_data << [floor, pos] }
    p = AdventOfCode::D1P1::Processor.new(cb)
    p.process('()())')
    expect(cb_data).to eq [
      [1, 1],
      [0, 2],
      [1, 3],
      [0, 4],
      [-1, 5]
    ]
  end
end
