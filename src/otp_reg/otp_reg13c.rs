#[doc = "Register `OTP_REG13C` reader"]
pub type R = crate::R<OtpReg13cSpec>;
#[doc = "Register `OTP_REG13C` writer"]
pub type W = crate::W<OtpReg13cSpec>;
#[doc = "Field `REGREGIONSECURE3STARTOFFSET` reader - REG_REGION_SECURE3_START_OFFSET"]
pub type Regregionsecure3startoffsetR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE3STARTOFFSET` writer - REG_REGION_SECURE3_START_OFFSET"]
pub type Regregionsecure3startoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGREGIONSECURE3SIZE` reader - REG_REGION_SECURE3_SIZE"]
pub type Regregionsecure3sizeR = crate::FieldReader<u16>;
#[doc = "Field `REGREGIONSECURE3SIZE` writer - REG_REGION_SECURE3_SIZE"]
pub type Regregionsecure3sizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_REGION_SECURE3_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure3startoffset(&self) -> Regregionsecure3startoffsetR {
        Regregionsecure3startoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE3_SIZE"]
    #[inline(always)]
    pub fn regregionsecure3size(&self) -> Regregionsecure3sizeR {
        Regregionsecure3sizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_REGION_SECURE3_START_OFFSET"]
    #[inline(always)]
    pub fn regregionsecure3startoffset(&mut self) -> Regregionsecure3startoffsetW<OtpReg13cSpec> {
        Regregionsecure3startoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_REGION_SECURE3_SIZE"]
    #[inline(always)]
    pub fn regregionsecure3size(&mut self) -> Regregionsecure3sizeW<OtpReg13cSpec> {
        Regregionsecure3sizeW::new(self, 16)
    }
}
#[doc = "OTP\\_REGION\\_SECURE\\_RANGE3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg13cSpec;
impl crate::RegisterSpec for OtpReg13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg13c::R`](R) reader structure"]
impl crate::Readable for OtpReg13cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg13c::W`](W) writer structure"]
impl crate::Writable for OtpReg13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG13C to value 0x0090_0000"]
impl crate::Resettable for OtpReg13cSpec {
    const RESET_VALUE: u32 = 0x0090_0000;
}
