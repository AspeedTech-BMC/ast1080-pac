#[doc = "Register `I2C00` reader"]
pub type R = crate::R<I2c00Spec>;
#[doc = "Register `I2C00` writer"]
pub type W = crate::W<I2c00Spec>;
#[doc = "Field `MSTEN` reader - MST_EN"]
pub type MstenR = crate::BitReader;
#[doc = "Field `MSTEN` writer - MST_EN"]
pub type MstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVEN` reader - SLV_EN"]
pub type SlvenR = crate::BitReader;
#[doc = "Field `SLVEN` writer - SLV_EN"]
pub type SlvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `DEBOUNCE` reader - DEBOUNCE"]
pub type DebounceR = crate::FieldReader;
#[doc = "Field `DEBOUNCE` writer - DEBOUNCE"]
pub type DebounceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSMODE` reader - HS_MODE"]
pub type HsmodeR = crate::BitReader;
#[doc = "Field `HSMODE` writer - HS_MODE"]
pub type HsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLPU` reader - SCL_PU"]
pub type SclpuR = crate::BitReader;
#[doc = "Field `SCLPU` writer - SCL_PU"]
pub type SclpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAPU` reader - SDA_PU"]
pub type SdapuR = crate::BitReader;
#[doc = "Field `SDAPU` writer - SDA_PU"]
pub type SdapuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVRNGA` reader - SLV_RNGA"]
pub type SlvrngaR = crate::BitReader;
#[doc = "Field `SLVRNGA` writer - SLV_RNGA"]
pub type SlvrngaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCLDRIVEEN` reader - SCL_DRIVE_EN"]
pub type ScldriveenR = crate::BitReader;
#[doc = "Field `SCLDRIVEEN` writer - SCL_DRIVE_EN"]
pub type ScldriveenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTIMSTDIS` reader - MULTI_MST_DIS"]
pub type MultimstdisR = crate::BitReader;
#[doc = "Field `MULTIMSTDIS` writer - MULTI_MST_DIS"]
pub type MultimstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORECOVERY` reader - AUTO_RECOVERY"]
pub type AutorecoveryR = crate::BitReader;
#[doc = "Field `AUTORECOVERY` writer - AUTO_RECOVERY"]
pub type AutorecoveryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORELEASE` reader - AUTO_RELEASE"]
pub type AutoreleaseR = crate::BitReader;
#[doc = "Field `AUTORELEASE` writer - AUTO_RELEASE"]
pub type AutoreleaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPKGRETRY` reader - MPKG_RETRY"]
pub type MpkgretryR = crate::FieldReader;
#[doc = "Field `MPKGRETRY` writer - MPKG_RETRY"]
pub type MpkgretryW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLVADDRSAVEEN` reader - SLV_ADDR_SAVE_EN"]
pub type SlvaddrsaveenR = crate::BitReader;
#[doc = "Field `SLVADDRSAVEEN` writer - SLV_ADDR_SAVE_EN"]
pub type SlvaddrsaveenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MST_EN"]
    #[inline(always)]
    pub fn msten(&self) -> MstenR {
        MstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLV_EN"]
    #[inline(always)]
    pub fn slven(&self) -> SlvenR {
        SlvenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DEBOUNCE"]
    #[inline(always)]
    pub fn debounce(&self) -> DebounceR {
        DebounceR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HS_MODE"]
    #[inline(always)]
    pub fn hsmode(&self) -> HsmodeR {
        HsmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCL_PU"]
    #[inline(always)]
    pub fn sclpu(&self) -> SclpuR {
        SclpuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SDA_PU"]
    #[inline(always)]
    pub fn sdapu(&self) -> SdapuR {
        SdapuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SLV_RNGA"]
    #[inline(always)]
    pub fn slvrnga(&self) -> SlvrngaR {
        SlvrngaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - SCL_DRIVE_EN"]
    #[inline(always)]
    pub fn scldriveen(&self) -> ScldriveenR {
        ScldriveenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MULTI_MST_DIS"]
    #[inline(always)]
    pub fn multimstdis(&self) -> MultimstdisR {
        MultimstdisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AUTO_RECOVERY"]
    #[inline(always)]
    pub fn autorecovery(&self) -> AutorecoveryR {
        AutorecoveryR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AUTO_RELEASE"]
    #[inline(always)]
    pub fn autorelease(&self) -> AutoreleaseR {
        AutoreleaseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - MPKG_RETRY"]
    #[inline(always)]
    pub fn mpkgretry(&self) -> MpkgretryR {
        MpkgretryR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SLV_ADDR_SAVE_EN"]
    #[inline(always)]
    pub fn slvaddrsaveen(&self) -> SlvaddrsaveenR {
        SlvaddrsaveenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MST_EN"]
    #[inline(always)]
    pub fn msten(&mut self) -> MstenW<I2c00Spec> {
        MstenW::new(self, 0)
    }
    #[doc = "Bit 1 - SLV_EN"]
    #[inline(always)]
    pub fn slven(&mut self) -> SlvenW<I2c00Spec> {
        SlvenW::new(self, 1)
    }
    #[doc = "Bits 4:5 - DEBOUNCE"]
    #[inline(always)]
    pub fn debounce(&mut self) -> DebounceW<I2c00Spec> {
        DebounceW::new(self, 4)
    }
    #[doc = "Bit 6 - HS_MODE"]
    #[inline(always)]
    pub fn hsmode(&mut self) -> HsmodeW<I2c00Spec> {
        HsmodeW::new(self, 6)
    }
    #[doc = "Bit 7 - SCL_PU"]
    #[inline(always)]
    pub fn sclpu(&mut self) -> SclpuW<I2c00Spec> {
        SclpuW::new(self, 7)
    }
    #[doc = "Bit 8 - SDA_PU"]
    #[inline(always)]
    pub fn sdapu(&mut self) -> SdapuW<I2c00Spec> {
        SdapuW::new(self, 8)
    }
    #[doc = "Bit 9 - SLV_RNGA"]
    #[inline(always)]
    pub fn slvrnga(&mut self) -> SlvrngaW<I2c00Spec> {
        SlvrngaW::new(self, 9)
    }
    #[doc = "Bit 14 - SCL_DRIVE_EN"]
    #[inline(always)]
    pub fn scldriveen(&mut self) -> ScldriveenW<I2c00Spec> {
        ScldriveenW::new(self, 14)
    }
    #[doc = "Bit 15 - MULTI_MST_DIS"]
    #[inline(always)]
    pub fn multimstdis(&mut self) -> MultimstdisW<I2c00Spec> {
        MultimstdisW::new(self, 15)
    }
    #[doc = "Bit 16 - AUTO_RECOVERY"]
    #[inline(always)]
    pub fn autorecovery(&mut self) -> AutorecoveryW<I2c00Spec> {
        AutorecoveryW::new(self, 16)
    }
    #[doc = "Bit 17 - AUTO_RELEASE"]
    #[inline(always)]
    pub fn autorelease(&mut self) -> AutoreleaseW<I2c00Spec> {
        AutoreleaseW::new(self, 17)
    }
    #[doc = "Bits 18:19 - MPKG_RETRY"]
    #[inline(always)]
    pub fn mpkgretry(&mut self) -> MpkgretryW<I2c00Spec> {
        MpkgretryW::new(self, 18)
    }
    #[doc = "Bit 20 - SLV_ADDR_SAVE_EN"]
    #[inline(always)]
    pub fn slvaddrsaveen(&mut self) -> SlvaddrsaveenW<I2c00Spec> {
        SlvaddrsaveenW::new(self, 20)
    }
}
#[doc = "Master/Slave Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c00Spec;
impl crate::RegisterSpec for I2c00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c00::R`](R) reader structure"]
impl crate::Readable for I2c00Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c00::W`](W) writer structure"]
impl crate::Writable for I2c00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C00 to value 0"]
impl crate::Resettable for I2c00Spec {}
