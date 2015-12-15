require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd10')

describe AdventOfCode::D10::LookSayGenerator do
  def sequence(s)
    p = AdventOfCode::D10::LookSayGenerator.new
    p.next(s)
  end

  it 'should produce the right values for example sequences' do
    expect(sequence('1')).to eq '11'
    expect(sequence('11')).to eq '21'
    expect(sequence('21')).to eq '1211'
    expect(sequence('1211')).to eq '111221'
    expect(sequence('111221')).to eq '312211'
  end

  it 'should produce the right values when iterated' do
    seq = '1'
    5.times { seq = sequence(seq) }
    expect(seq).to eq '312211'
  end
end
