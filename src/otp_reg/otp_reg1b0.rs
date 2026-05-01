#[doc = "Register `OTP_REG1B0` reader"]
pub type R = crate::R<OtpReg1b0Spec>;
#[doc = "Register `OTP_REG1B0` writer"]
pub type W = crate::W<OtpReg1b0Spec>;
#[doc = "Field `REGMASTERID0` reader - REG_MASTER_ID0"]
pub type Regmasterid0R = crate::FieldReader;
#[doc = "Field `REGMASTERID0` writer - REG_MASTER_ID0"]
pub type Regmasterid0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERID1` reader - REG_MASTER_ID1"]
pub type Regmasterid1R = crate::FieldReader;
#[doc = "Field `REGMASTERID1` writer - REG_MASTER_ID1"]
pub type Regmasterid1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERID2` reader - REG_MASTER_ID2"]
pub type Regmasterid2R = crate::FieldReader;
#[doc = "Field `REGMASTERID2` writer - REG_MASTER_ID2"]
pub type Regmasterid2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGMASTERID3` reader - REG_MASTER_ID3"]
pub type Regmasterid3R = crate::FieldReader;
#[doc = "Field `REGMASTERID3` writer - REG_MASTER_ID3"]
pub type Regmasterid3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_MASTER_ID0"]
    #[inline(always)]
    pub fn regmasterid0(&self) -> Regmasterid0R {
        Regmasterid0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_MASTER_ID1"]
    #[inline(always)]
    pub fn regmasterid1(&self) -> Regmasterid1R {
        Regmasterid1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_MASTER_ID2"]
    #[inline(always)]
    pub fn regmasterid2(&self) -> Regmasterid2R {
        Regmasterid2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_MASTER_ID3"]
    #[inline(always)]
    pub fn regmasterid3(&self) -> Regmasterid3R {
        Regmasterid3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_MASTER_ID0"]
    #[inline(always)]
    pub fn regmasterid0(&mut self) -> Regmasterid0W<OtpReg1b0Spec> {
        Regmasterid0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_MASTER_ID1"]
    #[inline(always)]
    pub fn regmasterid1(&mut self) -> Regmasterid1W<OtpReg1b0Spec> {
        Regmasterid1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_MASTER_ID2"]
    #[inline(always)]
    pub fn regmasterid2(&mut self) -> Regmasterid2W<OtpReg1b0Spec> {
        Regmasterid2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - REG_MASTER_ID3"]
    #[inline(always)]
    pub fn regmasterid3(&mut self) -> Regmasterid3W<OtpReg1b0Spec> {
        Regmasterid3W::new(self, 24)
    }
}
#[doc = "OTP\\_MASTER\\_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1b0Spec;
impl crate::RegisterSpec for OtpReg1b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1b0::R`](R) reader structure"]
impl crate::Readable for OtpReg1b0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1b0::W`](W) writer structure"]
impl crate::Writable for OtpReg1b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1B0 to value 0xffff_ffff"]
impl crate::Resettable for OtpReg1b0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
