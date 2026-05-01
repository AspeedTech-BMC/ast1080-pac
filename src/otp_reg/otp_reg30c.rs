#[doc = "Register `OTP_REG30C` reader"]
pub type R = crate::R<OtpReg30cSpec>;
#[doc = "Register `OTP_REG30C` writer"]
pub type W = crate::W<OtpReg30cSpec>;
#[doc = "Field `REGSWINFO3` reader - REG_SW_INFO3"]
pub type Regswinfo3R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO3` writer - REG_SW_INFO3"]
pub type Regswinfo3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO3"]
    #[inline(always)]
    pub fn regswinfo3(&self) -> Regswinfo3R {
        Regswinfo3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO3"]
    #[inline(always)]
    pub fn regswinfo3(&mut self) -> Regswinfo3W<OtpReg30cSpec> {
        Regswinfo3W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg30c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg30c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg30cSpec;
impl crate::RegisterSpec for OtpReg30cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg30c::R`](R) reader structure"]
impl crate::Readable for OtpReg30cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg30c::W`](W) writer structure"]
impl crate::Writable for OtpReg30cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG30C to value 0x03"]
impl crate::Resettable for OtpReg30cSpec {
    const RESET_VALUE: u32 = 0x03;
}
