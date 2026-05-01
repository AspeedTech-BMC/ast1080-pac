#[doc = "Register `OTP_REG10C` reader"]
pub type R = crate::R<OtpReg10cSpec>;
#[doc = "Register `OTP_REG10C` writer"]
pub type W = crate::W<OtpReg10cSpec>;
#[doc = "Field `REGREGIONOTPCTRLREN` reader - REG_REGION_OTP_CTRL_REN"]
pub type RegregionotpctrlrenR = crate::FieldReader;
#[doc = "Field `REGREGIONOTPCTRLREN` writer - REG_REGION_OTP_CTRL_REN"]
pub type RegregionotpctrlrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONOTPCTRLWEN` reader - REG_REGION_OTP_CTRL_WEN"]
pub type RegregionotpctrlwenR = crate::FieldReader;
#[doc = "Field `REGREGIONOTPCTRLWEN` writer - REG_REGION_OTP_CTRL_WEN"]
pub type RegregionotpctrlwenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONOTPCTRLLOCK` reader - REG_REGION_OTP_CTRL_LOCK"]
pub type RegregionotpctrllockR = crate::BitReader;
#[doc = "Field `REGREGIONOTPCTRLLOCK` writer - REG_REGION_OTP_CTRL_LOCK"]
pub type RegregionotpctrllockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_OTP_CTRL_REN"]
    #[inline(always)]
    pub fn regregionotpctrlren(&self) -> RegregionotpctrlrenR {
        RegregionotpctrlrenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_OTP_CTRL_WEN"]
    #[inline(always)]
    pub fn regregionotpctrlwen(&self) -> RegregionotpctrlwenR {
        RegregionotpctrlwenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_OTP_CTRL_LOCK"]
    #[inline(always)]
    pub fn regregionotpctrllock(&self) -> RegregionotpctrllockR {
        RegregionotpctrllockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_OTP_CTRL_REN"]
    #[inline(always)]
    pub fn regregionotpctrlren(&mut self) -> RegregionotpctrlrenW<OtpReg10cSpec> {
        RegregionotpctrlrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_OTP_CTRL_WEN"]
    #[inline(always)]
    pub fn regregionotpctrlwen(&mut self) -> RegregionotpctrlwenW<OtpReg10cSpec> {
        RegregionotpctrlwenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_OTP_CTRL_LOCK"]
    #[inline(always)]
    pub fn regregionotpctrllock(&mut self) -> RegregionotpctrllockW<OtpReg10cSpec> {
        RegregionotpctrllockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_OTP\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg10cSpec;
impl crate::RegisterSpec for OtpReg10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg10c::R`](R) reader structure"]
impl crate::Readable for OtpReg10cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg10c::W`](W) writer structure"]
impl crate::Writable for OtpReg10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG10C to value 0xffff"]
impl crate::Resettable for OtpReg10cSpec {
    const RESET_VALUE: u32 = 0xffff;
}
