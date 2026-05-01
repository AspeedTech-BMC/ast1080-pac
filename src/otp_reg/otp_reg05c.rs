#[doc = "Register `OTP_REG05C` reader"]
pub type R = crate::R<OtpReg05cSpec>;
#[doc = "Register `OTP_REG05C` writer"]
pub type W = crate::W<OtpReg05cSpec>;
#[doc = "Field `REGOTPADDRM2` reader - REG_OTP_ADDR_M2"]
pub type Regotpaddrm2R = crate::FieldReader<u16>;
#[doc = "Field `REGOTPADDRM2` writer - REG_OTP_ADDR_M2"]
pub type Regotpaddrm2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M2"]
    #[inline(always)]
    pub fn regotpaddrm2(&self) -> Regotpaddrm2R {
        Regotpaddrm2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M2"]
    #[inline(always)]
    pub fn regotpaddrm2(&mut self) -> Regotpaddrm2W<OtpReg05cSpec> {
        Regotpaddrm2W::new(self, 0)
    }
}
#[doc = "otp\\_addr\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg05cSpec;
impl crate::RegisterSpec for OtpReg05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg05c::R`](R) reader structure"]
impl crate::Readable for OtpReg05cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg05c::W`](W) writer structure"]
impl crate::Writable for OtpReg05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG05C to value 0"]
impl crate::Resettable for OtpReg05cSpec {}
