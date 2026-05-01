#[doc = "Register `OTP_REG03C` reader"]
pub type R = crate::R<OtpReg03cSpec>;
#[doc = "Register `OTP_REG03C` writer"]
pub type W = crate::W<OtpReg03cSpec>;
#[doc = "Field `REGOTPADDRM1` reader - REG_OTP_ADDR_M1"]
pub type Regotpaddrm1R = crate::FieldReader<u16>;
#[doc = "Field `REGOTPADDRM1` writer - REG_OTP_ADDR_M1"]
pub type Regotpaddrm1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M1"]
    #[inline(always)]
    pub fn regotpaddrm1(&self) -> Regotpaddrm1R {
        Regotpaddrm1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M1"]
    #[inline(always)]
    pub fn regotpaddrm1(&mut self) -> Regotpaddrm1W<OtpReg03cSpec> {
        Regotpaddrm1W::new(self, 0)
    }
}
#[doc = "otp\\_addr\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg03cSpec;
impl crate::RegisterSpec for OtpReg03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg03c::R`](R) reader structure"]
impl crate::Readable for OtpReg03cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg03c::W`](W) writer structure"]
impl crate::Writable for OtpReg03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG03C to value 0"]
impl crate::Resettable for OtpReg03cSpec {}
