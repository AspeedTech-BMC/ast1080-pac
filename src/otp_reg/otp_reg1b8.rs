#[doc = "Register `OTP_REG1B8` reader"]
pub type R = crate::R<OtpReg1b8Spec>;
#[doc = "Register `OTP_REG1B8` writer"]
pub type W = crate::W<OtpReg1b8Spec>;
#[doc = "Field `REGMASTERRID0` reader - REG_MASTER_RID0"]
pub type Regmasterrid0R = crate::FieldReader;
#[doc = "Field `REGMASTERRID0` writer - REG_MASTER_RID0"]
pub type Regmasterrid0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERRID1` reader - REG_MASTER_RID1"]
pub type Regmasterrid1R = crate::FieldReader;
#[doc = "Field `REGMASTERRID1` writer - REG_MASTER_RID1"]
pub type Regmasterrid1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERRID2` reader - REG_MASTER_RID2"]
pub type Regmasterrid2R = crate::FieldReader;
#[doc = "Field `REGMASTERRID2` writer - REG_MASTER_RID2"]
pub type Regmasterrid2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERRID3` reader - REG_MASTER_RID3"]
pub type Regmasterrid3R = crate::FieldReader;
#[doc = "Field `REGMASTERRID3` writer - REG_MASTER_RID3"]
pub type Regmasterrid3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_MASTER_RID0"]
    #[inline(always)]
    pub fn regmasterrid0(&self) -> Regmasterrid0R {
        Regmasterrid0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_MASTER_RID1"]
    #[inline(always)]
    pub fn regmasterrid1(&self) -> Regmasterrid1R {
        Regmasterrid1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_MASTER_RID2"]
    #[inline(always)]
    pub fn regmasterrid2(&self) -> Regmasterrid2R {
        Regmasterrid2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_MASTER_RID3"]
    #[inline(always)]
    pub fn regmasterrid3(&self) -> Regmasterrid3R {
        Regmasterrid3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_MASTER_RID0"]
    #[inline(always)]
    pub fn regmasterrid0(&mut self) -> Regmasterrid0W<OtpReg1b8Spec> {
        Regmasterrid0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_MASTER_RID1"]
    #[inline(always)]
    pub fn regmasterrid1(&mut self) -> Regmasterrid1W<OtpReg1b8Spec> {
        Regmasterrid1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_MASTER_RID2"]
    #[inline(always)]
    pub fn regmasterrid2(&mut self) -> Regmasterrid2W<OtpReg1b8Spec> {
        Regmasterrid2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - REG_MASTER_RID3"]
    #[inline(always)]
    pub fn regmasterrid3(&mut self) -> Regmasterrid3W<OtpReg1b8Spec> {
        Regmasterrid3W::new(self, 24)
    }
}
#[doc = "OTP\\_R\\_MASTER\\_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1b8Spec;
impl crate::RegisterSpec for OtpReg1b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1b8::R`](R) reader structure"]
impl crate::Readable for OtpReg1b8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1b8::W`](W) writer structure"]
impl crate::Writable for OtpReg1b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1B8 to value 0xffff_ffff"]
impl crate::Resettable for OtpReg1b8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
