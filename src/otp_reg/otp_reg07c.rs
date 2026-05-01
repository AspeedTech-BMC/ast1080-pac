#[doc = "Register `OTP_REG07C` reader"]
pub type R = crate::R<OtpReg07cSpec>;
#[doc = "Register `OTP_REG07C` writer"]
pub type W = crate::W<OtpReg07cSpec>;
#[doc = "Field `REGOTPADDRM3` reader - REG_OTP_ADDR_M3"]
pub type Regotpaddrm3R = crate::FieldReader<u16>;
#[doc = "Field `REGOTPADDRM3` writer - REG_OTP_ADDR_M3"]
pub type Regotpaddrm3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M3"]
    #[inline(always)]
    pub fn regotpaddrm3(&self) -> Regotpaddrm3R {
        Regotpaddrm3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_OTP_ADDR_M3"]
    #[inline(always)]
    pub fn regotpaddrm3(&mut self) -> Regotpaddrm3W<OtpReg07cSpec> {
        Regotpaddrm3W::new(self, 0)
    }
}
#[doc = "otp\\_addr\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg07cSpec;
impl crate::RegisterSpec for OtpReg07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg07c::R`](R) reader structure"]
impl crate::Readable for OtpReg07cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg07c::W`](W) writer structure"]
impl crate::Writable for OtpReg07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG07C to value 0"]
impl crate::Resettable for OtpReg07cSpec {}
