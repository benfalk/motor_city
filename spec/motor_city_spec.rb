# frozen_string_literal: true

RSpec.describe MotorCity do
  it "has a version number" do
    expect(MotorCity::VERSION).not_to be nil
  end

  describe "#connection_ok?" do
    subject { described_class.connection_ok? }
    it { is_expected.to be(true) }
  end

  describe "#connection" do
    subject { described_class.connection }
    it { is_expected.to be_a(::FFI::Pointer) }
    it { is_expected.to_not be_null }
  end
end
