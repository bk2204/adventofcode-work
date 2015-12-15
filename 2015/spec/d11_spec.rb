require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd11')

describe AdventOfCode::D11::PasswordGenerator do
  it 'should reject certain values' do
    p = AdventOfCode::D11::PasswordGenerator.new
    %w(hijklmmn abbceffg abbcegjk).each do |pass|
      expect(p.valid?(pass)).to eq false
    end
  end

  it 'should accept certain values' do
    p = AdventOfCode::D11::PasswordGenerator.new
    %w(abcdffaa ghjaabcc).each do |pass|
      expect(p.valid?(pass)).to eq true
    end
  end

  it 'should produce the right values for example passwords' do
    p = AdventOfCode::D11::PasswordGenerator.new
    expect(p.next('abcdefgh')).to eq 'abcdffaa'
    expect(p.next('ghijklmn')).to eq 'ghjaabcc'
  end
end
