#[doc = "Register `OTP_REG1B4` reader"]
pub type R = crate::R<OtpReg1b4Spec>;
#[doc = "Register `OTP_REG1B4` writer"]
pub type W = crate::W<OtpReg1b4Spec>;
#[doc = "Field `REGMASTERID4` reader - REG_MASTER_ID4"]
pub type Regmasterid4R = crate::FieldReader;
#[doc = "Field `REGMASTERID4` writer - REG_MASTER_ID4"]
pub type Regmasterid4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERID5` reader - REG_MASTER_ID5"]
pub type Regmasterid5R = crate::FieldReader;
#[doc = "Field `REGMASTERID5` writer - REG_MASTER_ID5"]
pub type Regmasterid5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERLOCK` reader - REG_MASTER_LOCK"]
pub type RegmasterlockR = crate::BitReader;
#[doc = "Field `REGMASTERLOCK` writer - REG_MASTER_LOCK"]
pub type RegmasterlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_MASTER_ID4"]
    #[inline(always)]
    pub fn regmasterid4(&self) -> Regmasterid4R {
        Regmasterid4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_MASTER_ID5"]
    #[inline(always)]
    pub fn regmasterid5(&self) -> Regmasterid5R {
        Regmasterid5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_MASTER_LOCK"]
    #[inline(always)]
    pub fn regmasterlock(&self) -> RegmasterlockR {
        RegmasterlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_MASTER_ID4"]
    #[inline(always)]
    pub fn regmasterid4(&mut self) -> Regmasterid4W<OtpReg1b4Spec> {
        Regmasterid4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_MASTER_ID5"]
    #[inline(always)]
    pub fn regmasterid5(&mut self) -> Regmasterid5W<OtpReg1b4Spec> {
        Regmasterid5W::new(self, 8)
    }
    #[doc = "Bit 31 - REG_MASTER_LOCK"]
    #[inline(always)]
    pub fn regmasterlock(&mut self) -> RegmasterlockW<OtpReg1b4Spec> {
        RegmasterlockW::new(self, 31)
    }
}
#[doc = "OTP\\_MASTER\\_ID\\_EXT\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1b4Spec;
impl crate::RegisterSpec for OtpReg1b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1b4::R`](R) reader structure"]
impl crate::Readable for OtpReg1b4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1b4::W`](W) writer structure"]
impl crate::Writable for OtpReg1b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1B4 to value 0xffff"]
impl crate::Resettable for OtpReg1b4Spec {
    const RESET_VALUE: u32 = 0xffff;
}
