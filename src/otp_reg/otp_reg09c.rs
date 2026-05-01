#[doc = "Register `OTP_REG09C` reader"]
pub type R = crate::R<OtpReg09cSpec>;
#[doc = "Register `OTP_REG09C` writer"]
pub type W = crate::W<OtpReg09cSpec>;
#[doc = "Field `REGOTPADDRM4` reader - REG_OTP_ADDR_M4"]
pub type Regotpaddrm4R = crate::FieldReader<u16>;
#[doc = "Field `REGOTPADDRM4` writer - REG_OTP_ADDR_M4"]
pub type Regotpaddrm4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M4"]
    #[inline(always)]
    pub fn regotpaddrm4(&self) -> Regotpaddrm4R {
        Regotpaddrm4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M4"]
    #[inline(always)]
    pub fn regotpaddrm4(&mut self) -> Regotpaddrm4W<OtpReg09cSpec> {
        Regotpaddrm4W::new(self, 0)
    }
}
#[doc = "otp\\_addr\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg09cSpec;
impl crate::RegisterSpec for OtpReg09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg09c::R`](R) reader structure"]
impl crate::Readable for OtpReg09cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg09c::W`](W) writer structure"]
impl crate::Writable for OtpReg09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG09C to value 0"]
impl crate::Resettable for OtpReg09cSpec {}
