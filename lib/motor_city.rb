# frozen_string_literal: true

require "dotenv/load"
require "ffi"
require_relative "motor_city/version"
require_relative "motor_city/post"
require_relative "motor_city/ffi"

# This is Motor City
module MotorCity
  class Error < StandardError; end

  # Connects to the database
  #
  # Defaults to `DATABASE_URL` env if no db url is provided
  #
  # @param db_url [String] db config url to use
  # @return [nil]
  def self.establish_connection(db_url = ENV.fetch("DATABASE_URL", ""))
    result_ptr = FFI.establish_connection(db_url)
    result = FFI::ConnectionResult.new(result_ptr)
    raise Error, result[:value].read_string unless result[:status].zero?

    @connection = result[:value]
    nil
  ensure
    FFI.free_result(result_ptr)
  end

  # Determines if the provided connection is working
  #
  # @return [Boolean]
  def self.connection_ok?
    return false if connection.nil?

    FFI.connection_ok(connection)
  end

  # The db connection if one has been established
  #
  # @return [::FFI::Pointer]
  def self.connection
    @connection
  end
end
