#[doc = "Register `I2C18` reader"]
pub type R = crate::R<I2c18Spec>;
#[doc = "Register `I2C18` writer"]
pub type W = crate::W<I2c18Spec>;
#[doc = "Field `MCMDSTART` reader - MCMD_START"]
pub type McmdstartR = crate::BitReader;
#[doc = "Field `MCMDSTART` writer - MCMD_START"]
pub type McmdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMDTXD` reader - MCMD_TXD"]
pub type McmdtxdR = crate::BitReader;
#[doc = "Field `MCMDTXD` writer - MCMD_TXD"]
pub type McmdtxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `MCMDRXD` reader - MCMD_RXD"]
pub type McmdrxdR = crate::BitReader;
#[doc = "Field `MCMDRXD` writer - MCMD_RXD"]
pub type McmdrxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMDRXLAST` reader - MCMD_RXLAST"]
pub type McmdrxlastR = crate::BitReader;
#[doc = "Field `MCMDRXLAST` writer - MCMD_RXLAST"]
pub type McmdrxlastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMDSTOP` reader - MCMD_STOP"]
pub type McmdstopR = crate::BitReader;
#[doc = "Field `MCMDSTOP` writer - MCMD_STOP"]
pub type McmdstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBUFTXEN` reader - MBUF_TX_EN"]
pub type MbuftxenR = crate::BitReader;
#[doc = "Field `MBUFTXEN` writer - MBUF_TX_EN"]
pub type MbuftxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBUFRXEN` reader - MBUF_RX_EN"]
pub type MbufrxenR = crate::BitReader;
#[doc = "Field `MBUFRXEN` writer - MBUF_RX_EN"]
pub type MbufrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTXEN` reader - MTX_EN"]
pub type MtxenR = crate::BitReader;
#[doc = "Field `MTXEN` writer - MTX_EN"]
pub type MtxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRXEN` reader - MRX_EN"]
pub type MrxenR = crate::BitReader;
#[doc = "Field `MRXEN` writer - MRX_EN"]
pub type MrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMDSMBUSEN` reader - MCMD_SMBUS_EN"]
pub type McmdsmbusenR = crate::BitReader;
#[doc = "Field `MCMDSMBUSEN` writer - MCMD_SMBUS_EN"]
pub type McmdsmbusenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSRECOVERY` reader - BUS_RECOVERY"]
pub type BusrecoveryR = crate::BitReader;
#[doc = "Field `BUSRECOVERY` writer - BUS_RECOVERY"]
pub type BusrecoveryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLGPO` reader - SCL_GPO"]
pub type SclgpoR = crate::BitReader;
#[doc = "Field `SCLGPO` writer - SCL_GPO"]
pub type SclgpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLGPOE` reader - SCL_GPOE"]
pub type SclgpoeR = crate::BitReader;
#[doc = "Field `SCLGPOE` writer - SCL_GPOE"]
pub type SclgpoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAGPO` reader - SDA_GPO"]
pub type SdagpoR = crate::BitReader;
#[doc = "Field `SDAGPO` writer - SDA_GPO"]
pub type SdagpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAGPOE` reader - SDA_GPOE"]
pub type SdagpoeR = crate::BitReader;
#[doc = "Field `SDAGPOE` writer - SDA_GPOE"]
pub type SdagpoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPKGEN` reader - MPKG_EN"]
pub type MpkgenR = crate::BitReader;
#[doc = "Field `MPKGEN` writer - MPKG_EN"]
pub type MpkgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTCODE` reader - MST_CODE"]
pub type MstcodeR = crate::FieldReader;
#[doc = "Field `MSTCODE` writer - MST_CODE"]
pub type MstcodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `MDSTSADDR` reader - MDST_SADDR"]
pub type MdstsaddrR = crate::FieldReader;
#[doc = "Field `MDSTSADDR` writer - MDST_SADDR"]
pub type MdstsaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - MCMD_START"]
    #[inline(always)]
    pub fn mcmdstart(&self) -> McmdstartR {
        McmdstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCMD_TXD"]
    #[inline(always)]
    pub fn mcmdtxd(&self) -> McmdtxdR {
        McmdtxdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCMD_RXD"]
    #[inline(always)]
    pub fn mcmdrxd(&self) -> McmdrxdR {
        McmdrxdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MCMD_RXLAST"]
    #[inline(always)]
    pub fn mcmdrxlast(&self) -> McmdrxlastR {
        McmdrxlastR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MCMD_STOP"]
    #[inline(always)]
    pub fn mcmdstop(&self) -> McmdstopR {
        McmdstopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MBUF_TX_EN"]
    #[inline(always)]
    pub fn mbuftxen(&self) -> MbuftxenR {
        MbuftxenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MBUF_RX_EN"]
    #[inline(always)]
    pub fn mbufrxen(&self) -> MbufrxenR {
        MbufrxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MTX_EN"]
    #[inline(always)]
    pub fn mtxen(&self) -> MtxenR {
        MtxenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MRX_EN"]
    #[inline(always)]
    pub fn mrxen(&self) -> MrxenR {
        MrxenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MCMD_SMBUS_EN"]
    #[inline(always)]
    pub fn mcmdsmbusen(&self) -> McmdsmbusenR {
        McmdsmbusenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BUS_RECOVERY"]
    #[inline(always)]
    pub fn busrecovery(&self) -> BusrecoveryR {
        BusrecoveryR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCL_GPO"]
    #[inline(always)]
    pub fn sclgpo(&self) -> SclgpoR {
        SclgpoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCL_GPOE"]
    #[inline(always)]
    pub fn sclgpoe(&self) -> SclgpoeR {
        SclgpoeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SDA_GPO"]
    #[inline(always)]
    pub fn sdagpo(&self) -> SdagpoR {
        SdagpoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDA_GPOE"]
    #[inline(always)]
    pub fn sdagpoe(&self) -> SdagpoeR {
        SdagpoeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MPKG_EN"]
    #[inline(always)]
    pub fn mpkgen(&self) -> MpkgenR {
        MpkgenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - MST_CODE"]
    #[inline(always)]
    pub fn mstcode(&self) -> MstcodeR {
        MstcodeR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - MDST_SADDR"]
    #[inline(always)]
    pub fn mdstsaddr(&self) -> MdstsaddrR {
        MdstsaddrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MCMD_START"]
    #[inline(always)]
    pub fn mcmdstart(&mut self) -> McmdstartW<I2c18Spec> {
        McmdstartW::new(self, 0)
    }
    #[doc = "Bit 1 - MCMD_TXD"]
    #[inline(always)]
    pub fn mcmdtxd(&mut self) -> McmdtxdW<I2c18Spec> {
        McmdtxdW::new(self, 1)
    }
    #[doc = "Bit 3 - MCMD_RXD"]
    #[inline(always)]
    pub fn mcmdrxd(&mut self) -> McmdrxdW<I2c18Spec> {
        McmdrxdW::new(self, 3)
    }
    #[doc = "Bit 4 - MCMD_RXLAST"]
    #[inline(always)]
    pub fn mcmdrxlast(&mut self) -> McmdrxlastW<I2c18Spec> {
        McmdrxlastW::new(self, 4)
    }
    #[doc = "Bit 5 - MCMD_STOP"]
    #[inline(always)]
    pub fn mcmdstop(&mut self) -> McmdstopW<I2c18Spec> {
        McmdstopW::new(self, 5)
    }
    #[doc = "Bit 6 - MBUF_TX_EN"]
    #[inline(always)]
    pub fn mbuftxen(&mut self) -> MbuftxenW<I2c18Spec> {
        MbuftxenW::new(self, 6)
    }
    #[doc = "Bit 7 - MBUF_RX_EN"]
    #[inline(always)]
    pub fn mbufrxen(&mut self) -> MbufrxenW<I2c18Spec> {
        MbufrxenW::new(self, 7)
    }
    #[doc = "Bit 8 - MTX_EN"]
    #[inline(always)]
    pub fn mtxen(&mut self) -> MtxenW<I2c18Spec> {
        MtxenW::new(self, 8)
    }
    #[doc = "Bit 9 - MRX_EN"]
    #[inline(always)]
    pub fn mrxen(&mut self) -> MrxenW<I2c18Spec> {
        MrxenW::new(self, 9)
    }
    #[doc = "Bit 10 - MCMD_SMBUS_EN"]
    #[inline(always)]
    pub fn mcmdsmbusen(&mut self) -> McmdsmbusenW<I2c18Spec> {
        McmdsmbusenW::new(self, 10)
    }
    #[doc = "Bit 11 - BUS_RECOVERY"]
    #[inline(always)]
    pub fn busrecovery(&mut self) -> BusrecoveryW<I2c18Spec> {
        BusrecoveryW::new(self, 11)
    }
    #[doc = "Bit 12 - SCL_GPO"]
    #[inline(always)]
    pub fn sclgpo(&mut self) -> SclgpoW<I2c18Spec> {
        SclgpoW::new(self, 12)
    }
    #[doc = "Bit 13 - SCL_GPOE"]
    #[inline(always)]
    pub fn sclgpoe(&mut self) -> SclgpoeW<I2c18Spec> {
        SclgpoeW::new(self, 13)
    }
    #[doc = "Bit 14 - SDA_GPO"]
    #[inline(always)]
    pub fn sdagpo(&mut self) -> SdagpoW<I2c18Spec> {
        SdagpoW::new(self, 14)
    }
    #[doc = "Bit 15 - SDA_GPOE"]
    #[inline(always)]
    pub fn sdagpoe(&mut self) -> SdagpoeW<I2c18Spec> {
        SdagpoeW::new(self, 15)
    }
    #[doc = "Bit 16 - MPKG_EN"]
    #[inline(always)]
    pub fn mpkgen(&mut self) -> MpkgenW<I2c18Spec> {
        MpkgenW::new(self, 16)
    }
    #[doc = "Bits 17:19 - MST_CODE"]
    #[inline(always)]
    pub fn mstcode(&mut self) -> MstcodeW<I2c18Spec> {
        MstcodeW::new(self, 17)
    }
    #[doc = "Bits 24:30 - MDST_SADDR"]
    #[inline(always)]
    pub fn mdstsaddr(&mut self) -> MdstsaddrW<I2c18Spec> {
        MdstsaddrW::new(self, 24)
    }
}
#[doc = "Master Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c18Spec;
impl crate::RegisterSpec for I2c18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c18::R`](R) reader structure"]
impl crate::Readable for I2c18Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c18::W`](W) writer structure"]
impl crate::Writable for I2c18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C18 to value 0"]
impl crate::Resettable for I2c18Spec {}
