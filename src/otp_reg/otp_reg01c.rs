#[doc = "Register `OTP_REG01C` reader"]
pub type R = crate::R<OtpReg01cSpec>;
#[doc = "Register `OTP_REG01C` writer"]
pub type W = crate::W<OtpReg01cSpec>;
#[doc = "Field `REGOTPADDRM0` reader - REG_OTP_ADDR_M0"]
pub type Regotpaddrm0R = crate::FieldReader<u16>;
#[doc = "Field `REGOTPADDRM0` writer - REG_OTP_ADDR_M0"]
pub type Regotpaddrm0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M0"]
    #[inline(always)]
    pub fn regotpaddrm0(&self) -> Regotpaddrm0R {
        Regotpaddrm0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M0"]
    #[inline(always)]
    pub fn regotpaddrm0(&mut self) -> Regotpaddrm0W<OtpReg01cSpec> {
        Regotpaddrm0W::new(self, 0)
    }
}
#[doc = "otp\\_addr\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg01cSpec;
impl crate::RegisterSpec for OtpReg01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg01c::R`](R) reader structure"]
impl crate::Readable for OtpReg01cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg01c::W`](W) writer structure"]
impl crate::Writable for OtpReg01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG01C to value 0"]
impl crate::Resettable for OtpReg01cSpec {}
